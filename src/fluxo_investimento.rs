use crate::decimal::Decimal;
use crate::impostos_e_taxas::{ImpostosETaxas, SaldoAtivo};
use crate::quantidade_ou_valor::QuantidadeOuValor;
use crate::titulo::Titulo;
use anyhow::{ensure, Context, Result};
use chrono::{Days, NaiveDate};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct FluxoInvestimento<'a> {
    titulo: &'a Titulo,
    ultimo_dia: Option<NaiveDate>,
    saldo_quantidade: Decimal<2>,
    eventos: Vec<EventoInvestimento>,
    saldos_ativos: VecDeque<SaldoAtivo>,
}

#[derive(Debug, Clone)]
pub struct EventoInvestimento {
    pub dia: NaiveDate,
    pub tipo: TipoEvento,
    pub valor_bruto: Decimal<2>,
    pub quantidade: Decimal<2>,
    pub preco: Decimal<6>,
    pub saldo_quantidade: Decimal<2>,
    pub impostos_e_taxas: ImpostosETaxas,
    pub valor_liquido: Decimal<2>,
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
            saldos_ativos: VecDeque::new(),
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
            valor_bruto: valor,
            quantidade,
            preco: preco.as_decimal(),
            saldo_quantidade: self.saldo_quantidade,
            impostos_e_taxas: ImpostosETaxas::zero(),
            valor_liquido: valor,
        });
        self.saldos_ativos.push_front(SaldoAtivo {
            dia,
            preco_original: preco,
            quantidade,
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

        // Encontra os saldos que foram resgatados
        let mut resgastes = vec![];
        let mut quantidade_restante = quantidade;
        while quantidade_restante > Decimal::zero() {
            let saldo_ativo = self
                .saldos_ativos
                .pop_back()
                .context("precisa haver pelo menos um saldo ativo")?;

            let resgate = if saldo_ativo.quantidade > quantidade_restante {
                // Resgaste parcial
                self.saldos_ativos.push_back(
                    saldo_ativo.com_quantidade(saldo_ativo.quantidade - quantidade_restante),
                );
                saldo_ativo.com_quantidade(quantidade_restante)
            } else {
                saldo_ativo
            };

            resgastes.push(resgate);

            quantidade_restante -= resgate.quantidade;
        }

        let impostos_e_taxas = ImpostosETaxas::calcular_para_resgaste(&resgastes, preco, dia);

        self.eventos.push(EventoInvestimento {
            dia,
            tipo: TipoEvento::Venda,
            valor_bruto: valor,
            quantidade,
            preco: preco.as_decimal(),
            saldo_quantidade: self.saldo_quantidade,
            impostos_e_taxas,
            valor_liquido: valor - impostos_e_taxas.total(),
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
                let impostos_e_taxas = ImpostosETaxas::calcular_para_cupom(
                    self.saldos_ativos.make_contiguous(),
                    preco,
                    dia,
                );

                self.eventos.push(EventoInvestimento {
                    dia,
                    tipo: TipoEvento::Cupom,
                    valor_bruto: valor,
                    quantidade: self.saldo_quantidade,
                    preco,
                    saldo_quantidade: self.saldo_quantidade,
                    impostos_e_taxas,
                    valor_liquido: valor - impostos_e_taxas.total(),
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
