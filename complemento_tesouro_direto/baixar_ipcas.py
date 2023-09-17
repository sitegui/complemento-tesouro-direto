import pandas
import requests

if __name__ == '__main__':
    # Baixa os dados de IPCA do IBGE
    # Dados: https://sidra.ibge.gov.br/tabela/1737
    # Documentação da API: https://apisidra.ibge.gov.br/home/ajuda
    dados_json = requests.get('https://apisidra.ibge.gov.br/values/t/1737/n1/all/p/all/v/2266/f/c?formato=json').json()

    # O primeiro elemento é um cabeçalho
    dados = pandas.DataFrame(dados_json[1:], columns=['D2C', 'V']).rename(columns={
        'D2C': 'mes',
        'V': 'indice',
    })
    dados = dados.assign(
        mes=pandas.to_datetime(dados['mes'], format='%Y%m'),
        indice=dados['indice'].astype('float'),
    )

    dados.to_pickle('dados/ipcas.pkl.zip')