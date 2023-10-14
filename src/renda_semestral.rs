use crate::decimal::Decimal;
use crate::fluxo_investimento::{FluxoInvestimento, TipoEvento};
use crate::inflacao::Inflacao;
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
        let num_semestres = (fim - inicio).num_days() / 180;
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

    pub fn total(&self) -> Decimal<2> {
        self.semestres
            .iter()
            .fold(Decimal::zero(), |total, &semestre| total + semestre)
    }

    pub fn variancia(&self) -> Decimal<2> {
        let media = self.total().as_float() / self.semestres.len() as f64;

        let diferencas_ao_quadrado = self
            .semestres
            .iter()
            .map(|&semestre| (semestre.as_float() - media).powi(2))
            .sum::<f64>();

        let variancia = (diferencas_ao_quadrado / (self.semestres.len() as f64 - 1.0)).sqrt();

        Decimal::new(variancia)
    }
}
