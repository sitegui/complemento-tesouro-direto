use crate::decimal::Decimal;
use crate::serie_temporal::SerieTemporal;
use chrono::NaiveDate;
use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug, Clone)]
pub struct Inflacao {
    valores: SerieTemporal<Decimal<2>>,
}

impl Inflacao {
    /// Baixa os dados de IPCA do IBGE
    /// Dados: https://sidra.ibge.gov.br/tabela/1737
    /// Documentação da API: https://apisidra.ibge.gov.br/home/ajuda
    pub fn baixar() -> Self {
        #[derive(Debug, Clone, Deserialize)]
        struct Elemento {
            /// Formato "yyyymm", exemplo "201308"
            #[serde(rename = "D2C")]
            mes: String,
            /// Formato float de duas casas significativas como string, exemplo "3725.9500000000000"
            #[serde(rename = "V")]
            indice: String,
        }

        let elementos: Vec<Elemento> = reqwest::blocking::get(
            "https://apisidra.ibge.gov.br/values/t/1737/n1/all/p/200001-210001/v/2266/f/c?formato=json",
        )
        .unwrap()
        .json()
        .unwrap();

        // O primeiro elemento é um cabeçalho
        let valores = elementos[1..]
            .iter()
            .map(|elemento| {
                assert_eq!(elemento.mes.len(), 6);
                let ano = elemento.mes[0..4].parse::<i32>().unwrap();
                let mes = elemento.mes[4..6].parse::<u32>().unwrap();
                let dia = NaiveDate::from_ymd_opt(ano, mes, 1).unwrap();

                let indice = Decimal::new(elemento.indice.parse::<f32>().unwrap());

                (dia, indice)
            })
            .collect();

        Inflacao {
            valores: SerieTemporal::new(valores),
        }
    }

    pub fn baixar_com_cache() -> Self {
        let cache = "dados/inflacao.json";

        match fs::read_to_string(cache) {
            Err(erro) if erro.kind() == ErrorKind::NotFound => {
                let inflacao = Inflacao::baixar();

                fs::write(cache, serde_json::to_string(&inflacao.valores).unwrap()).unwrap();

                inflacao
            }
            Ok(cacheado) => Inflacao {
                valores: serde_json::from_str(&cacheado).unwrap(),
            },
            Err(erro) => {
                panic!("erro ao ler cache: {}", erro);
            }
        }
    }
}
