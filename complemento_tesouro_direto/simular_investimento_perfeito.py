import pandas
from datetime import timedelta, date

import pandas as pd
from sympy import Symbol, nsolve, solve, sympify


def main():
    precos = pandas.read_pickle('dados/precos.pkl.zip')
    cupons = pandas.read_pickle('dados/cupons.pkl.zip')
    ipcas = pandas.read_pickle('dados/ipcas.pkl.zip')

    tipo = 'Tesouro IPCA+ com Juros Semestrais'
    vencimento = '2020-08-15'
    precos = precos[
        (precos['tipo'] == tipo) & (precos['vencimento'] == vencimento) & (precos['preco_unitario_venda'] > 0)]
    cupons = cupons[(cupons['tipo'] == tipo) & (cupons['vencimento'] == vencimento)]

    preco_compra_por_dia = precos.set_index('dia')['preco_unitario_compra'].sort_index()
    preco_venda_por_dia = precos.set_index('dia')['preco_unitario_venda'].sort_index()
    cupom_por_dia = cupons.set_index('dia')['preco_unitario'].sort_index()
    ipca_por_dia = ipcas.set_index('mes')['indice'].sort_index()

    valor_inicial = 100_000
    data_inicial = '2012-08-17'
    numero_semestres = 14
    espera_para_vender = timedelta(days=2)

    primeiro_indice_cupom = cupom_por_dia.index.searchsorted(data_inicial, side='right')
    cupons_ganhos = cupom_por_dia.iloc[primeiro_indice_cupom:primeiro_indice_cupom + numero_semestres]
    assert len(cupons_ganhos) == numero_semestres

    preco_unitario_inicial = preco_compra_por_dia.loc[data_inicial]
    quantidade = valor_inicial / preco_unitario_inicial
    resgate_corrigido = Symbol('resgate_corrigido')
    ipca_inicial = ipca_por_dia.asof(data_inicial)

    semestres = []
    for dia, preco_unitario in cupons_ganhos.items():
        quantidade_antes_da_venda = quantidade
        cupom = quantidade * preco_unitario
        ipca = ipca_por_dia.asof(dia)
        inflacao = ipca / ipca_inicial
        venda = resgate_corrigido * inflacao - cupom
        dia_venda = dia + espera_para_vender
        while dia_venda not in preco_venda_por_dia:
            dia_venda += timedelta(days=1)
        preco_venda_unitario = preco_venda_por_dia.loc[dia_venda]
        quantidade = quantidade - venda / preco_venda_unitario

        semestres.append({
            'quantidade': quantidade_antes_da_venda,
            'dia_cupom': dia,
            'cupom': cupom,
            'inflacao': 100 * (inflacao - 1),
            'dia_venda': dia_venda,
            'preco_venda_unitario': preco_venda_unitario,
            'venda': venda,
        })

    resgate_corrigido_subs = solve(quantidade, resgate_corrigido, dict=True)[0]
    for semestre in semestres:
        semestre['quantidade'] = sympify(semestre['quantidade']).evalf(subs=resgate_corrigido_subs)
        semestre['cupom'] = sympify(semestre['cupom']).evalf(subs=resgate_corrigido_subs)
        semestre['venda'] = sympify(semestre['venda']).evalf(subs=resgate_corrigido_subs)

    semestres = pandas.DataFrame(semestres)
    semestres['resgate'] = semestres['cupom'] + semestres['venda']
    semestres['resgate_corrigido'] = semestres['resgate'] / (1 + semestres['inflacao'] / 100)
    semestres['quantidade_venda'] = semestres['venda'] / semestres['preco_venda_unitario']

    idade_cupom = semestres['dia_cupom'] - pd.to_datetime(data_inicial)
    semestres['imposto_cupom'] = semestres['cupom'] * idade_cupom.apply(get_taxa_imposto)

    idade_venda = semestres['dia_venda'] - pd.to_datetime(data_inicial)
    renda_venda = semestres['quantidade_venda'] * (semestres['preco_venda_unitario'] - preco_unitario_inicial).clip(lower=0)
    semestres['imposto_venda'] = renda_venda * idade_venda.apply(get_taxa_imposto)

    semestres['imposto'] = semestres['imposto_cupom'] + semestres['imposto_venda']
    semestres['resgate_liquido'] = semestres['resgate'] - semestres['imposto']
    semestres['resgate_liquido_corrigido'] = semestres['resgate_liquido'] / (1 + semestres['inflacao'] / 100)

    print(semestres.to_csv())


def get_taxa_imposto(tempo: timedelta) -> float:
    if tempo < timedelta(days=180):
        return 0.225
    elif tempo < timedelta(days=360):
        return 0.20
    elif tempo < timedelta(days=720):
        return 0.175
    else:
        return 0.15


if __name__ == '__main__':
    main()
