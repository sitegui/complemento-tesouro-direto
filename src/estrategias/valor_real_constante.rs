use crate::decimal::Decimal;
use crate::estrategias::Estrategia;
use crate::fluxo_investimento::FluxoInvestimento;
use crate::inflacao::Inflacao;
use crate::quantidade_ou_valor::QuantidadeOuValor;
use anyhow::{ensure, Context, Result};
use chrono::{Duration, NaiveDate};

/// Aplica uma estratégia de vender cada período o mesmo valor real (ou seja, corrigido pela inflação)
#[derive(Debug, Clone)]
pub struct ValorRealConstante<'a> {
    inflacao: &'a Inflacao,
    frequencia_venda: Duration,
}

impl<'a> ValorRealConstante<'a> {
    pub fn new(inflacao: &'a Inflacao, frequencia_venda: Duration) -> Self {
        Self {
            inflacao,
            frequencia_venda,
        }
    }
}

impl<'a> Estrategia for ValorRealConstante<'a> {
    fn nome(&self) -> String {
        format!("valor-real-constante({})", self.frequencia_venda)
    }

    fn aplicar<'b>(
        &self,
        fluxo: FluxoInvestimento<'b>,
        dia_inicio: NaiveDate,
        periodo: Duration,
    ) -> Result<FluxoInvestimento<'b>> {
        let num_vendas = periodo.num_seconds() / self.frequencia_venda.num_seconds();

        // Tenta venda um valor real a cada período. Retorna `None` se o valor foi muito alto
        let tentar_venda = |venda_real: Decimal<2>| {
            let mut fluxo = fluxo.clone();

            for indice in 0..num_vendas {
                let dia_venda = dia_inicio + periodo * indice as i32;
                let venda = self.inflacao.corrigir(venda_real, dia_inicio, dia_venda);
                let resultado =
                    fluxo.vender_a_partir_de(dia_venda, QuantidadeOuValor::Valor(venda));

                if resultado.is_err() {
                    return None;
                }
            }

            Some(fluxo)
        };

        let mut minimo = Decimal::<2>::zero();
        let mut maximo = fluxo.saldo_valor(dia_inicio);

        let mut melhor_fluxo =
            tentar_venda(minimo).context("a venda mínima precisa ser factível")?;
        ensure!(tentar_venda(maximo).is_none());

        loop {
            let meio = (minimo + maximo) / 2.0;
            if meio == minimo || meio == maximo {
                break;
            }

            match tentar_venda(meio) {
                None => {
                    maximo = meio;
                }
                Some(fluxo) => {
                    minimo = meio;
                    melhor_fluxo = fluxo;
                }
            }
        }

        Ok(melhor_fluxo)
    }
}
