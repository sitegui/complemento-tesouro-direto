use crate::decimal::Decimal;
use crate::tipo_titulo::TipoTitulo;
use crate::titulo::{Titulo, TituloBuilder};
use anyhow::Context;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

pub fn ler_titulos() -> Vec<Titulo> {
    let mut titulos = Titulos::default();

    ler_precos(&mut titulos);
    ler_cupons(&mut titulos);
    ler_vencimentos(&mut titulos);

    titulos.build()
}

#[derive(Debug, Default)]
struct Titulos(HashMap<(TipoTitulo, NaiveDate), TituloBuilder>);

impl Titulos {
    fn get_mut(&mut self, tipo: TipoTitulo, vencimento: NaiveDate) -> &mut TituloBuilder {
        self.0
            .entry((tipo, vencimento))
            .or_insert_with(|| TituloBuilder::new(tipo, vencimento))
    }

    fn build(self) -> Vec<Titulo> {
        self.0.into_values().map(|titulo| titulo.build()).collect()
    }
}

fn ler_precos(titulos: &mut Titulos) {
    #[derive(Debug, Deserialize)]
    struct Linha {
        #[serde(rename = "Tipo Titulo")]
        tipo: TipoTitulo,
        #[serde(rename = "Data Vencimento", deserialize_with = "ler_dia")]
        vencimento: NaiveDate,
        #[serde(rename = "Data Base", deserialize_with = "ler_dia")]
        dia: NaiveDate,
        #[serde(rename = "PU Compra Manha", deserialize_with = "ler_decimal")]
        preco_compra: Decimal<2>,
        #[serde(rename = "PU Venda Manha", deserialize_with = "ler_decimal")]
        preco_venda: Decimal<2>,
    }

    let mut leitor = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("dados/PrecoTaxaTesouroDireto.csv")
        .unwrap();

    for linha in leitor.deserialize::<Linha>() {
        let linha = linha.unwrap();

        let titulo = titulos.get_mut(linha.tipo, linha.vencimento);

        if linha.preco_compra > Decimal::zero() {
            titulo.adicionar_preco_compra(linha.dia, linha.preco_compra);
        }
        if linha.preco_venda > Decimal::zero() {
            titulo.adicionar_preco_venda(linha.dia, linha.preco_venda);
        }
    }
}

fn ler_cupons(titulos: &mut Titulos) {
    #[derive(Debug, Deserialize)]
    struct Linha {
        #[serde(rename = "Tipo Titulo")]
        tipo: TipoTitulo,
        #[serde(rename = "Vencimento do Titulo", deserialize_with = "ler_dia")]
        vencimento: NaiveDate,
        #[serde(rename = "Data Resgate", deserialize_with = "ler_dia")]
        dia: NaiveDate,
        #[serde(rename = "PU", deserialize_with = "ler_decimal")]
        cupom: Decimal<6>,
    }

    let mut leitor = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("dados/CupomJurosTesouroDireto.csv")
        .unwrap();

    for linha in leitor.deserialize::<Linha>() {
        let linha = linha.unwrap();

        titulos
            .get_mut(linha.tipo, linha.vencimento)
            .adicionar_cupom(linha.dia, linha.cupom);
    }
}

fn ler_vencimentos(titulos: &mut Titulos) {
    #[derive(Debug, Deserialize)]
    struct Linha {
        #[serde(rename = "Tipo Titulo")]
        tipo: TipoTitulo,
        #[serde(rename = "Vencimento do Titulo", deserialize_with = "ler_dia")]
        vencimento: NaiveDate,
        #[serde(rename = "Data Resgate", deserialize_with = "ler_dia")]
        dia: NaiveDate,
        #[serde(rename = "PU", deserialize_with = "ler_decimal")]
        preco_vencimento: Decimal<6>,
    }

    let mut leitor = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("dados/VencimentosTesouroDireto.csv")
        .unwrap();

    for linha in leitor.deserialize::<Linha>() {
        let linha = linha.unwrap();

        titulos
            .get_mut(linha.tipo, linha.vencimento)
            .set_preco_vencimento(linha.preco_vencimento);
    }
}

fn ler_dia<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let texto = String::deserialize(deserializer)?;

    NaiveDate::parse_from_str(&texto, "%d/%m/%Y").map_err(Error::custom)
}

fn ler_decimal<'de, const N: u8, D>(deserializer: D) -> Result<Decimal<N>, D::Error>
where
    D: Deserializer<'de>,
{
    let texto = String::deserialize(deserializer)?;
    let valor = texto
        .replace(',', ".")
        .parse::<f64>()
        .with_context(|| format!("não pude ler o decimal '{}'", texto))
        .map_err(Error::custom)?;

    Ok(Decimal::new(valor))
}
