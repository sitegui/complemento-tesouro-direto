use crate::decimal::Decimal;
use crate::titulo::{Titulo, TituloBuilder};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

mod decimal;
mod serie_temporal;
mod titulo;

fn main() {
    let titulos = ler_titulos("Tesouro IPCA+ com Juros Semestrais");

    eprintln!("titulos.len() = {:?}", titulos.len());
}

fn ler_titulos(tipo: &str) -> Vec<Titulo> {
    let mut titulos = HashMap::new();

    ler_precos(tipo, &mut titulos);
    ler_cupons(tipo, &mut titulos);
    ler_vencimentos(tipo, &mut titulos);

    titulos.into_values().map(|titulo| titulo.build()).collect()
}

fn ler_precos(tipo: &str, titulos: &mut HashMap<NaiveDate, TituloBuilder>) {
    #[derive(Debug, Deserialize)]
    struct Linha {
        #[serde(rename = "Tipo Titulo")]
        tipo: String,
        #[serde(rename = "Data Vencimento", deserialize_with = "ler_dia")]
        vencimento: NaiveDate,
        #[serde(rename = "Data Base", deserialize_with = "ler_dia")]
        dia: NaiveDate,
        #[serde(rename = "PU Compra Manha")]
        preco_compra: Decimal<2>,
        #[serde(rename = "PU Venda Manha")]
        preco_venda: Decimal<2>,
    }

    let mut leitor = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("dados/PrecoTaxaTesouroDireto.csv")
        .unwrap();

    for linha in leitor.deserialize::<Linha>() {
        let linha = linha.unwrap();

        if linha.tipo == tipo {
            let titulo = titulos
                .entry(linha.vencimento)
                .or_insert_with(|| TituloBuilder::new(linha.vencimento));

            if linha.preco_compra > Decimal::zero() {
                titulo.adicionar_preco_compra(linha.dia, linha.preco_compra);
            }
            if linha.preco_venda > Decimal::zero() {
                titulo.adicionar_preco_venda(linha.dia, linha.preco_venda);
            }
        }
    }
}

fn ler_cupons(tipo: &str, titulos: &mut HashMap<NaiveDate, TituloBuilder>) {
    #[derive(Debug, Deserialize)]
    struct Linha {
        #[serde(rename = "Tipo Titulo")]
        tipo: String,
        #[serde(rename = "Vencimento do Titulo", deserialize_with = "ler_dia")]
        vencimento: NaiveDate,
        #[serde(rename = "Data Resgate", deserialize_with = "ler_dia")]
        dia: NaiveDate,
        #[serde(rename = "PU")]
        cupom: Decimal<6>,
    }

    let mut leitor = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("dados/CupomJurosTesouroDireto.csv")
        .unwrap();

    for linha in leitor.deserialize::<Linha>() {
        let linha = linha.unwrap();

        if linha.tipo == tipo {
            titulos
                .entry(linha.vencimento)
                .or_insert_with(|| TituloBuilder::new(linha.vencimento))
                .adicionar_cupom(linha.dia, linha.cupom);
        }
    }
}

fn ler_vencimentos(tipo: &str, titulos: &mut HashMap<NaiveDate, TituloBuilder>) {
    #[derive(Debug, Deserialize)]
    struct Linha {
        #[serde(rename = "Tipo Titulo")]
        tipo: String,
        #[serde(rename = "Vencimento do Titulo", deserialize_with = "ler_dia")]
        vencimento: NaiveDate,
        #[serde(rename = "Data Resgate", deserialize_with = "ler_dia")]
        dia: NaiveDate,
        #[serde(rename = "PU")]
        preco_vencimento: Decimal<6>,
    }

    let mut leitor = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("dados/VencimentosTesouroDireto.csv")
        .unwrap();

    for linha in leitor.deserialize::<Linha>() {
        let linha = linha.unwrap();

        if linha.tipo == tipo {
            titulos
                .entry(linha.vencimento)
                .or_insert_with(|| TituloBuilder::new(linha.vencimento))
                .set_preco_vencimento(linha.preco_vencimento);
        }
    }
}

fn ler_dia<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let texto = String::deserialize(deserializer)?;

    NaiveDate::parse_from_str(&texto, "%d/%m/%Y").map_err(Error::custom)
}
