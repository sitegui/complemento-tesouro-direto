use crate::decimal::Decimal;
use crate::fluxo_investimento::{FluxoInvestimento, TipoEvento};
use crate::inflacao::Inflacao;
use anyhow::{ensure, Context, Result};
use chrono::NaiveDate;

/// Representa a renda semestral real obtida num dado fluxo
#[derive(Debug, Clone)]
pub struct RendaReal {
    semestres: Vec<Decimal<2>>,
}

impl RendaReal {
    pub fn new(
        inflacao: &Inflacao,
        fluxo: &FluxoInvestimento,
        inicio: NaiveDate,
        fim: NaiveDate,
    ) -> Self {
        let num_semestres = ((fim - inicio).num_days()) as f64 / 180.0;
        let num_semestres = num_semestres.ceil() as i64;
        let mut semestres = vec![Decimal::<2>::zero(); num_semestres as usize];

        for evento in fluxo.eventos() {
            match evento.tipo {
                TipoEvento::Compra => {}
                TipoEvento::Venda | TipoEvento::Cupom => {
                    let semestre = (evento.dia - inicio).num_days().max(0) / 180;
                    let renda_real = inflacao.corrigir(evento.valor_liquido, evento.dia, inicio);
                    semestres[semestre as usize] += renda_real;
                }
            }
        }

        RendaReal { semestres }
    }

    pub fn media_de_rendas(rendas: &[Self]) -> Result<Self> {
        let mut semestres = rendas
            .get(0)
            .context("expected at least one element")?
            .semestres
            .clone();
        for renda in rendas.iter().skip(1) {
            ensure!(renda.semestres.len() == semestres.len());
            for (a, &b) in semestres.iter_mut().zip(&renda.semestres) {
                *a += b;
            }
        }

        for semestre in &mut semestres {
            *semestre /= rendas.len() as f64;
        }

        Ok(RendaReal { semestres })
    }

    pub fn media(&self) -> Decimal<2> {
        let soma = self
            .semestres
            .iter()
            .map(|&semestre| semestre.as_float())
            .sum::<f64>();

        Decimal::new(soma / self.semestres.len() as f64)
    }

    pub fn variancia(&self) -> Decimal<2> {
        let media = self.media().as_float();

        let diferencas_ao_quadrado = self
            .semestres
            .iter()
            .map(|&semestre| (semestre.as_float() - media).powi(2))
            .sum::<f64>();

        let variancia = (diferencas_ao_quadrado / (self.semestres.len() as f64 - 1.0)).sqrt();

        Decimal::new(variancia)
    }
}
