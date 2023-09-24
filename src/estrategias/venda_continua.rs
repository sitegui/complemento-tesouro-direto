use crate::decimal::Decimal;
use crate::fluxo_investimento::FluxoInvestimento;
use crate::quantidade_ou_valor::QuantidadeOuValor;
use crate::titulo::Titulo;
use chrono::{Months, NaiveDate};

/// Aplica uma estratégia simples de vender cada mês a mesma quantidade
pub fn venda_continua(
    titulo: &Titulo,
    dia_inicio: NaiveDate,
    valor_inicio: Decimal<2>,
    num_meses: u32,
) -> FluxoInvestimento {
    let mut fluxo = FluxoInvestimento::new(titulo);

    fluxo
        .comprar_a_partir_de(dia_inicio, QuantidadeOuValor::Valor(valor_inicio))
        .unwrap();

    for num_mes in 0..num_meses {
        let meses_restantes = num_meses - num_mes;
        let quantidade_venda = fluxo.saldo_quantidade() / meses_restantes as f64;
        fluxo
            .vender_a_partir_de(
                dia_inicio + Months::new(num_mes),
                QuantidadeOuValor::Quantidade(quantidade_venda),
            )
            .unwrap();
    }

    fluxo
}
