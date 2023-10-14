use crate::decimal::Decimal;
use crate::estrategias::quantidade_constante::QuantidadeConstante;
use crate::estrategias::valor_real_constante::ValorRealConstante;
use crate::estrategias::Estrategia;
use crate::fluxo_investimento::FluxoInvestimento;
use crate::inflacao::Inflacao;
use crate::ler_titulos::ler_titulos;
use crate::quantidade_ou_valor::QuantidadeOuValor;
use crate::renda_semestral::RendaReal;
use crate::titulo::Titulo;
use anyhow::Context;
use anyhow::Result;
use chrono::{Duration, NaiveDate};
use std::collections::HashMap;

mod decimal;
mod estrategias;
mod fluxo_investimento;
mod inflacao;
mod ler_titulos;
mod quantidade_ou_valor;
mod renda_semestral;
mod serie_temporal;
mod tipo_titulo;
mod titulo;

fn main() -> Result<()> {
    let tempo_minimo = Duration::days(365 * 2);
    let valor_inicio = Decimal::<2>::new(100_000.0);
    let carencia = Duration::days(180);

    let inflacao = Inflacao::baixar_com_cache();
    let titulos = ler_titulos();

    let titulos = titulos_com_dados_suficientes(&titulos, tempo_minimo);

    let estrategias: Vec<Box<dyn Estrategia>> = vec![
        Box::new(QuantidadeConstante::new(Duration::days(30))),
        Box::new(QuantidadeConstante::new(Duration::days(180))),
        Box::new(ValorRealConstante::new(&inflacao, Duration::days(30))),
        Box::new(ValorRealConstante::new(&inflacao, Duration::days(180))),
    ];

    for titulo in titulos {
        println!(
            "{:?} {}: {} a {}",
            titulo.tipo, titulo.vencimento, titulo.inicio_dado, titulo.fim_dado
        );

        let resultados = testar_estrategias(
            titulo,
            titulo.inicio_dado,
            valor_inicio,
            tempo_minimo,
            carencia,
            &estrategias,
        )?;

        for (nome, fluxo) in resultados {
            println!("{}", nome);
            for evento in fluxo.eventos() {
                println!(
                    "{} {:?} {} {}",
                    evento.dia, evento.tipo, evento.valor, evento.saldo_quantidade
                );
            }

            println!(
                "{:?}",
                RendaReal::new(
                    &inflacao,
                    &fluxo,
                    titulo.inicio_dado,
                    titulo.inicio_dado + tempo_minimo
                )
            );
        }
    }

    Ok(())
}

fn titulos_com_dados_suficientes(titulos: &[Titulo], tempo_minimo: Duration) -> Vec<&Titulo> {
    titulos
        .iter()
        .filter(|titulo| titulo.fim_dado - titulo.inicio_dado > tempo_minimo)
        .collect()
}

fn testar_estrategias<'a>(
    titulo: &'a Titulo,
    dia_inicio: NaiveDate,
    valor_inicio: Decimal<2>,
    periodo: Duration,
    carencia: Duration,
    estrategias: &[Box<dyn Estrategia + '_>],
) -> Result<HashMap<String, FluxoInvestimento<'a>>> {
    let mut fluxo = FluxoInvestimento::new(titulo);
    fluxo
        .comprar_a_partir_de(dia_inicio, QuantidadeOuValor::Valor(valor_inicio))
        .context("compra inicial precisa ser possível")?;

    let resultados = estrategias
        .iter()
        .filter_map(|estrategia| {
            let resultado = estrategia
                .aplicar(fluxo.clone(), dia_inicio + carencia, periodo - carencia)
                .ok()?;
            Some((estrategia.nome(), resultado))
        })
        .collect();

    Ok(resultados)
}
