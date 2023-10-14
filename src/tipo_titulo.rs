use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum TipoTitulo {
    #[serde(rename = "Tesouro Educa+")]
    Educa,
    #[serde(rename = "Tesouro IGPM+ com Juros Semestrais")]
    IgpmComJurosSemestrais,
    #[serde(rename = "Tesouro IPCA+")]
    Ipca,
    #[serde(rename = "Tesouro IPCA+ com Juros Semestrais")]
    IpcaComJurosSemestrais,
    #[serde(rename = "Tesouro Prefixado")]
    Prefixado,
    #[serde(rename = "Tesouro Prefixado com Juros Semestrais")]
    PrefixadoComJurosSemestrais,
    #[serde(rename = "Tesouro Renda+ Aposentadoria Extra")]
    RendaAposentadoriaExtra,
    #[serde(rename = "Tesouro Selic")]
    Selic,
}

impl TipoTitulo {
    pub fn tem_juros_semestrais(self) -> bool {
        match self {
            TipoTitulo::IpcaComJurosSemestrais
            | TipoTitulo::PrefixadoComJurosSemestrais
            | TipoTitulo::IgpmComJurosSemestrais => true,
            TipoTitulo::Educa
            | TipoTitulo::Ipca
            | TipoTitulo::Prefixado
            | TipoTitulo::RendaAposentadoriaExtra
            | TipoTitulo::Selic => false,
        }
    }
}
