import pandas
from datetime import timedelta

if __name__ == '__main__':
    # Detecta quais títulos são validos para nossa análise do passado. Eles precisam ter dados de preços e pagamento
    # de cupons por pelo menos 8 anos
    precos = pandas.read_pickle('dados/precos.pkl.zip')
    cupons = pandas.read_pickle('dados/cupons.pkl.zip')

    tipo = 'Tesouro IPCA+ com Juros Semestrais'
    precos = precos[precos['tipo'] == tipo]
    cupons = cupons[cupons['tipo'] == tipo]

    intervalo_precos = precos.groupby('vencimento')['dia'].agg(lambda dias: dias.max() - dias.min())
    intervalo_cupons = cupons.groupby('vencimento')['dia'].agg(lambda dias: dias.max() - dias.min())

    intervalo_minimo = timedelta(days=8 * 365)

    precos_validos = intervalo_precos.index[intervalo_precos > intervalo_minimo]
    cupons_validos = intervalo_cupons.index[intervalo_cupons > intervalo_minimo]

    validos = sorted(set(precos_validos) & set(cupons_validos))
    print(validos)
