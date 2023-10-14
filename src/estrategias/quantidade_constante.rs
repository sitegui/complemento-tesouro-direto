use crate::estrategias::Estrategia;
use crate::fluxo_investimento::FluxoInvestimento;
use crate::quantidade_ou_valor::QuantidadeOuValor;
use anyhow::Result;
use chrono::{Duration, NaiveDate};

/// Aplica uma estratégia simples de vender cada período a mesma quantidade
#[derive(Debug, Clone)]
pub struct QuantidadeConstante {
    frequencia_venda: Duration,
}

impl QuantidadeConstante {
    pub fn new(frequencia_venda: Duration) -> Self {
        Self { frequencia_venda }
    }
}

impl Estrategia for QuantidadeConstante {
    fn nome(&self) -> String {
        format!("quantidade-constante({})", self.frequencia_venda)
    }

    fn aplicar<'a>(
        &self,
        mut fluxo: FluxoInvestimento<'a>,
        dia_inicio: NaiveDate,
        periodo: Duration,
    ) -> Result<FluxoInvestimento<'a>> {
        let num_vendas = periodo.num_seconds() / self.frequencia_venda.num_seconds();

        for indice in 0..num_vendas {
            let vendas_restantes = num_vendas - indice;
            let quantidade_venda = fluxo.saldo_quantidade() / vendas_restantes as f64;
            fluxo.vender_a_partir_de(
                dia_inicio + self.frequencia_venda * indice as i32,
                QuantidadeOuValor::Quantidade(quantidade_venda),
            )?;
        }

        Ok(fluxo)
    }
}
