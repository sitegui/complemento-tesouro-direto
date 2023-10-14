use crate::decimal::Decimal;
use crate::serie_temporal::SerieTemporal;
use crate::tipo_titulo::TipoTitulo;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Titulo {
    pub tipo: TipoTitulo,
    pub vencimento: NaiveDate,
    /// Inclusivo
    pub inicio_dado: NaiveDate,
    /// Exclusivo
    pub fim_dado: NaiveDate,
    pub preco_venda: SerieTemporal<Decimal<2>>,
    pub preco_compra: SerieTemporal<Decimal<2>>,
    pub cupom: SerieTemporal<Decimal<6>>,
    pub preco_vencimento: Option<Decimal<6>>,
}

#[derive(Debug, Clone)]
pub struct TituloBuilder {
    tipo: TipoTitulo,
    vencimento: NaiveDate,
    preco_venda: Vec<(NaiveDate, Decimal<2>)>,
    preco_compra: Vec<(NaiveDate, Decimal<2>)>,
    cupom: Vec<(NaiveDate, Decimal<6>)>,
    preco_vencimento: Option<Decimal<6>>,
}

impl TituloBuilder {
    pub fn new(tipo: TipoTitulo, vencimento: NaiveDate) -> Self {
        Self {
            tipo,
            vencimento,
            preco_venda: Default::default(),
            preco_compra: Default::default(),
            cupom: Default::default(),
            preco_vencimento: Default::default(),
        }
    }

    pub fn adicionar_preco_venda(&mut self, dia: NaiveDate, valor: Decimal<2>) {
        self.preco_venda.push((dia, valor));
    }

    pub fn adicionar_preco_compra(&mut self, dia: NaiveDate, valor: Decimal<2>) {
        self.preco_compra.push((dia, valor));
    }

    pub fn adicionar_cupom(&mut self, dia: NaiveDate, valor: Decimal<6>) {
        self.cupom.push((dia, valor));
    }

    pub fn set_preco_vencimento(&mut self, valor: Decimal<6>) {
        self.preco_vencimento = Some(valor);
    }

    pub fn build(self) -> Titulo {
        let preco_venda = SerieTemporal::new(self.preco_venda);
        let preco_compra = SerieTemporal::new(self.preco_compra);
        let cupom = SerieTemporal::new(self.cupom);

        let mut inicio_dado = preco_venda
            .primeiro()
            .unwrap()
            .0
            .max(preco_compra.primeiro().unwrap().0);

        let mut fim_dado = preco_venda
            .ultimo()
            .unwrap()
            .0
            .min(preco_compra.ultimo().unwrap().0);

        if self.tipo.tem_juros_semestrais() {
            inicio_dado = inicio_dado.max(cupom.primeiro().unwrap().0);
            fim_dado = fim_dado.min(cupom.ultimo().unwrap().0);
        }

        Titulo {
            tipo: self.tipo,
            vencimento: self.vencimento,
            inicio_dado,
            fim_dado,
            preco_venda,
            preco_compra,
            cupom,
            preco_vencimento: self.preco_vencimento,
        }
    }
}
