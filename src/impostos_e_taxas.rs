use crate::decimal::Decimal;
use chrono::NaiveDate;

/// Implementa as regras explicadas em:
/// <https://www.tesourodireto.com.br/blog/quais-sao-os-impostos-e-taxas-ao-investir-no-td.htm>
#[derive(Debug, Clone, Copy)]
pub struct ImpostosETaxas {
    pub imposto_operacao_financeira: Decimal<2>,
    pub imposto_renda: Decimal<2>,
    pub taxa_custodia: Decimal<2>,
}

#[derive(Debug, Clone, Copy)]
pub struct SaldoAtivo {
    pub dia: NaiveDate,
    pub preco_original: Decimal<2>,
    pub quantidade: Decimal<2>,
}

impl ImpostosETaxas {
    pub fn zero() -> Self {
        ImpostosETaxas {
            imposto_operacao_financeira: Decimal::zero(),
            imposto_renda: Decimal::zero(),
            taxa_custodia: Decimal::zero(),
        }
    }

    pub fn calcular_para_resgaste(
        resgates: &[SaldoAtivo],
        preco_atual: Decimal<2>,
        dia_atual: NaiveDate,
    ) -> Self {
        let mut imposto_operacao_financeira = Decimal::zero();
        let mut imposto_renda = Decimal::zero();
        let mut taxa_custodia = Decimal::zero();

        for resgate in resgates {
            let renda_unitaria = (preco_atual - resgate.preco_original).max(Decimal::zero());
            let mut renda = resgate.quantidade * renda_unitaria;
            let idade = (dia_atual - resgate.dia).num_days();

            let desconto = renda * aliquota_imposto_operacao_financeira(idade);
            imposto_operacao_financeira += desconto;
            renda -= imposto_operacao_financeira;

            imposto_renda += renda * aliquota_imposto_renda(idade);

            // Para simplificar, a taxa de custódia é calculada com o preço original
            taxa_custodia +=
                resgate.quantidade * resgate.preco_original * aliquota_taxa_custodia(idade);
        }

        ImpostosETaxas {
            imposto_operacao_financeira,
            imposto_renda,
            taxa_custodia,
        }
    }

    pub fn calcular_para_cupom(
        saldos: &[SaldoAtivo],
        preco_cupom: Decimal<6>,
        dia_atual: NaiveDate,
    ) -> Self {
        let mut imposto_renda = Decimal::zero();

        for saldo in saldos {
            let renda = saldo.quantidade * preco_cupom;
            let idade = (dia_atual - saldo.dia).num_days();

            imposto_renda += renda * aliquota_imposto_renda(idade);
        }

        ImpostosETaxas {
            imposto_operacao_financeira: Decimal::zero(),
            imposto_renda,
            // Para simplificar, a taxa de custódia só é cobrada no resgate
            taxa_custodia: Decimal::zero(),
        }
    }

    pub fn total(self) -> Decimal<2> {
        self.imposto_operacao_financeira + self.imposto_renda + self.taxa_custodia
    }
}

impl SaldoAtivo {
    pub fn com_quantidade(self, quantidade: Decimal<2>) -> Self {
        SaldoAtivo {
            dia: self.dia,
            preco_original: self.preco_original,
            quantidade,
        }
    }
}

fn aliquota_imposto_operacao_financeira(dias: i64) -> f64 {
    if dias >= 30 {
        return 0.0;
    }

    const ALIQUOTAS: [f64; 30] = [
        1.0, 0.96, 0.93, 0.90, 0.86, 0.83, 0.80, 0.76, 0.73, 0.70, 0.66, 0.63, 0.60, 0.56, 0.53,
        0.50, 0.46, 0.43, 0.40, 0.36, 0.33, 0.30, 0.26, 0.23, 0.20, 0.16, 0.13, 0.10, 0.6, 0.3,
    ];

    ALIQUOTAS[dias as usize]
}

fn aliquota_imposto_renda(dias: i64) -> f64 {
    if dias <= 180 {
        0.225
    } else if dias <= 360 {
        0.2
    } else if dias <= 720 {
        0.175
    } else {
        0.15
    }
}

fn aliquota_taxa_custodia(dias: i64) -> f64 {
    const TAXA_ANUAL: f64 = 0.002;

    TAXA_ANUAL * dias as f64 / 365.0
}
