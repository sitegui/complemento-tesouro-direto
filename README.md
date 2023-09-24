# Complemento de renda com Tesouro direto

Esse repositório irá analisar como usar títulos do Tesouro Direto como complemento de renda, pensando especialmente em
um investimento de médio prazo que irá consumir todo o capital investido.

O fluxo de investimento é:

1. investir uma soma inicial `I` num título IPCA+ com juros semestrais
2. a cada semestre, além dos cupom de juros semestral recebido, uma parte do investimento será vendido a preço de
  mercado.

Os objetivos são:

1. obter um valor líquido corrigido de inflação semestral constante durante um período total de `N` semestres.
2. ao final dos `N` semestres, zerar o saldo investigo

## Preparando ambiente

1. Instale `pyenv`
2. Compile a versão de Python usada nesse projeto com `pyenv install 3.11`
3. Crie o ambiente de dependências com `pyenv virtualenv 3.11 complemento-tesouro-direto`
4. Instale as dependências com `pip install -r requirements.txt`

Baixe os dados do site oficial do tesouro direto numa pasta `dados`:
- [Taxas dos Títulos Ofertados pelo Tesouro Direto](http://www.tesourotransparente.gov.br/ckan/dataset/taxas-dos-titulos-ofertados-pelo-tesouro-direto/resource/796d2059-14e9-44e3-80c9-2d9e30b405c1)
- [Pagamento de Cupom de Juros do Tesouro Direto](http://www.tesourotransparente.gov.br/ckan/dataset/resgates-do-tesouro-direto/resource/de2af5cf-9dbd-4566-b933-da6871cce030)
- [Vencimentos do Tesouro Direto](http://www.tesourotransparente.gov.br/ckan/dataset/resgates-do-tesouro-direto/resource/9180ec46-5d73-49ab-bd26-f16e2b323f74)

Execute `python -m complemento_tesouro_direto.limpar_dados`

## Análise do passado

Para entender as entranhas do tesouro direto, vou assumir um investimento num período do passado, assim posso usar os
valores reais para entender a estratégia usada.

IPCA+ vencendo em 2020-08-15
investimento inicial de 100 000 em 