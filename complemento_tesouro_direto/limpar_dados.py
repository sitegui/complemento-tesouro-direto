import pandas

if __name__ == '__main__':
    # Processa os dados baixados do site oficial num formato mais simples de ler e manipular com pandas

    pandas.read_csv(
        'dados/PrecoTaxaTesouroDireto.csv',
        sep=';',
        parse_dates=['Data Vencimento', 'Data Base'],
        dayfirst=True,
        decimal=',',
    ).rename(columns={
        'Tipo Titulo': 'tipo',
        'Data Vencimento': 'vencimento',
        'Data Base': 'dia',
        'Taxa Compra Manha': 'taxa_compra',
        'Taxa Venda Manha': 'taxa_venda',
        'PU Compra Manha': 'preco_unitario_compra',
        'PU Venda Manha': 'preco_unitario_venda',
        'PU Base Manha': 'preco_unitario_base',
    }).to_pickle('dados/precos.pkl.zip')

    pandas.read_csv(
        'dados/CupomJurosTesouroDireto.csv',
        sep=';',
        parse_dates=['Vencimento do Titulo', 'Data Resgate'],
        dayfirst=True,
        decimal=',',
    ).rename(columns={
        'Tipo Titulo': 'tipo',
        'Vencimento do Titulo': 'vencimento',
        'Data Resgate': 'dia',
        'PU': 'preco_unitario',
        'Quantidade': 'quantidade',
        'Valor': 'total',
    }).to_pickle('dados/cupons.pkl.zip')