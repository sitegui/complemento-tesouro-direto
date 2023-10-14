use crate::decimal::Decimal;
use crate::quantidade_ou_valor::QuantidadeOuValor;
use crate::titulo::Titulo;
use anyhow::{ensure, Context, Result};
use chrono::{Days, NaiveDate};

#[derive(Debug, Clone)]
pub struct FluxoInvestimento<'a> {
    titulo: &'a Titulo,
    ultimo_dia: Option<NaiveDate>,
    saldo_quantidade: Decimal<2>,
    eventos: Vec<EventoInvestimento>,
}

#[derive(Debug, Clone)]
pub struct EventoInvestimento {
    dia: NaiveDate,
    tipo: TipoEvento,
    valor: Decimal<2>,
    quantidade: Decimal<2>,
    preco: Decimal<6>,
    saldo_quantidade: Decimal<2>,
}

#[derive(Debug, Clone)]
pub enum TipoEvento {
    Compra,
    Venda,
    Cupom,
}

impl<'a> FluxoInvestimento<'a> {
    pub fn new(titulo: &'a Titulo) -> Self {
        Self {
            titulo,
            ultimo_dia: None,
            saldo_quantidade: Decimal::zero(),
            eventos: Default::default(),
        }
    }

    pub fn comprar_a_partir_de(
        &mut self,
        dia_minimo: NaiveDate,
        qv: QuantidadeOuValor,
    ) -> Result<()> {
        self.validar_dia(dia_minimo)?;

        let (dia, preco) = self
            .titulo
            .preco_compra
            .iter_comecando_em(dia_minimo)
            .next()
            .context("não tem como mais comprar esse título")?;
        self.ganhar_cupons(dia);

        let (quantidade, valor) = qv.quantidade_e_valor(preco);
        self.saldo_quantidade += quantidade;
        self.eventos.push(EventoInvestimento {
            dia,
            tipo: TipoEvento::Compra,
            valor,
            quantidade,
            preco: preco.as_decimal(),
            saldo_quantidade: self.saldo_quantidade,
        });

        Ok(())
    }

    pub fn vender_a_partir_de(
        &mut self,
        dia_minimo: NaiveDate,
        qv: QuantidadeOuValor,
    ) -> Result<()> {
        self.validar_dia(dia_minimo)?;

        let (dia, preco) = self
            .titulo
            .preco_venda
            .iter_comecando_em(dia_minimo)
            .next()
            .context("não tem como mais vender esse título")?;
        self.ganhar_cupons(dia);

        let (quantidade, valor) = qv.quantidade_e_valor(preco);
        ensure!(self.saldo_quantidade >= quantidade);
        self.saldo_quantidade -= quantidade;
        self.eventos.push(EventoInvestimento {
            dia,
            tipo: TipoEvento::Venda,
            valor,
            quantidade,
            preco: preco.as_decimal(),
            saldo_quantidade: self.saldo_quantidade,
        });

        Ok(())
    }

    /// Ganha os cupons até o dado dia, inclusive
    pub fn ganhar_cupons(&mut self, ate_dia: NaiveDate) {
        if let Some(ultimo_dia) = self.ultimo_dia {
            let proximo_dia = ultimo_dia + Days::new(1);
            for (dia, preco) in self.titulo.cupom.iter_comecando_em(proximo_dia) {
                if dia > ate_dia {
                    break;
                }

                let valor = self.saldo_quantidade * preco;
                self.eventos.push(EventoInvestimento {
                    dia,
                    tipo: TipoEvento::Cupom,
                    valor,
                    quantidade: self.saldo_quantidade,
                    preco,
                    saldo_quantidade: self.saldo_quantidade,
                });
            }

            self.ultimo_dia = Some(ate_dia.max(ultimo_dia));
        } else {
            self.ultimo_dia = Some(ate_dia);
        }
    }

    pub fn saldo_quantidade(&self) -> Decimal<2> {
        self.saldo_quantidade
    }

    pub fn saldo_valor(&self, dia: NaiveDate) -> Decimal<2> {
        self.titulo
            .preco_venda
            .valor_atual(dia)
            .expect("o título não pode ser vendido antes de existir")
            * self.saldo_quantidade
    }

    pub fn eventos(&self) -> &[EventoInvestimento] {
        &self.eventos
    }

    fn validar_dia(&self, dia: NaiveDate) -> Result<()> {
        if let Some(ultimo) = self.ultimo_dia {
            ensure!(dia >= ultimo);
        }
        Ok(())
    }
}
