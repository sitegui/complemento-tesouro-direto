use crate::fluxo_investimento::FluxoInvestimento;
use anyhow::Result;
use chrono::{Duration, NaiveDate};

pub mod quantidade_constante;
pub mod valor_real_constante;

pub trait Estrategia {
    fn nome(&self) -> String;

    fn aplicar<'a>(
        &self,
        fluxo: FluxoInvestimento<'a>,
        dia_inicio: NaiveDate,
        periodo: Duration,
    ) -> Result<FluxoInvestimento<'a>>;
}
