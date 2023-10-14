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
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::HashMap;

mod decimal;
mod estrategias;
mod fluxo_investimento;
mod impostos_e_taxas;
mod inflacao;
mod ler_titulos;
mod quantidade_ou_valor;
mod renda_semestral;
mod serie_temporal;
mod tipo_titulo;
mod titulo;

fn main() -> Result<()> {
    let periodo_total = Duration::days(365 * 10);
    let valor_inicio = Decimal::<2>::new(100_000.0);
    let passo_inicio = Duration::days(7);
    let carencia = Duration::days(180);

    let inflacao = Inflacao::baixar_com_cache();
    let titulos = ler_titulos();

    let titulos = titulos_com_dados_suficientes(&titulos, periodo_total);

    let estrategias: Vec<Box<dyn Estrategia>> = vec![
        Box::new(QuantidadeConstante::new(Duration::days(30))),
        Box::new(QuantidadeConstante::new(Duration::days(180))),
        Box::new(ValorRealConstante::new(&inflacao, Duration::days(30))),
        Box::new(ValorRealConstante::new(&inflacao, Duration::days(180))),
    ];

    for titulo in titulos {
        println!(
            "\n# {} {} (dados de {} até {})",
            titulo.tipo, titulo.vencimento, titulo.inicio_dado, titulo.fim_dado
        );

        let resultados = testar_estrategias_para_titulo(
            &inflacao,
            titulo,
            periodo_total,
            passo_inicio,
            valor_inicio,
            carencia,
            &estrategias,
        )?;
        let resultados = resultados
            .into_iter()
            .sorted_by_key(|(_, cada)| Reverse(cada.renda_media.media()));
        println!(
            "Média de {} testes",
            resultados.as_slice()[0].1.fluxos.len()
        );
        for (estrategia, resultados) in resultados {
            println!(
                "{} ~ {}: {}",
                resultados.renda_media.media(),
                resultados.renda_media.variancia(),
                estrategia,
            );
        }
    }

    Ok(())
}

#[derive(Debug)]
struct ResultadoDeEstrategia<'a> {
    fluxo: FluxoInvestimento<'a>,
    renda: RendaReal,
}

#[derive(Debug)]
struct ResultadosDeEstrategia<'a> {
    fluxos: Vec<FluxoInvestimento<'a>>,
    renda_media: RendaReal,
}

fn titulos_com_dados_suficientes(titulos: &[Titulo], tempo_minimo: Duration) -> Vec<&Titulo> {
    titulos
        .iter()
        .filter(|titulo| titulo.fim_dado - titulo.inicio_dado > tempo_minimo)
        .collect()
}

fn testar_estrategias_para_titulo<'a>(
    inflacao: &Inflacao,
    titulo: &'a Titulo,
    periodo_total: Duration,
    passo_inicio: Duration,
    valor_inicio: Decimal<2>,
    carencia: Duration,
    estrategias: &[Box<dyn Estrategia + '_>],
) -> Result<HashMap<String, ResultadosDeEstrategia<'a>>> {
    let mut inicio_analise = titulo.inicio_dado;
    let mut resultados = HashMap::<_, Vec<_>>::new();

    while inicio_analise + periodo_total <= titulo.fim_dado {
        let resultados_para_fluxo = testar_estrategias_para_fluxo(
            inflacao,
            titulo,
            titulo.inicio_dado,
            valor_inicio,
            periodo_total,
            carencia,
            &estrategias,
        )?;

        for (estrategia, fluxo) in resultados_para_fluxo {
            resultados.entry(estrategia).or_default().push(fluxo);
        }

        inicio_analise += passo_inicio;
    }

    let resultados = resultados
        .into_iter()
        .map(|(estrategia, resultados_de_estrategia)| {
            let (fluxos, rendas) = resultados_de_estrategia
                .into_iter()
                .map(|cada| (cada.fluxo, cada.renda))
                .unzip::<_, _, Vec<_>, Vec<_>>();

            let resultados_de_estrategia = ResultadosDeEstrategia {
                renda_media: RendaReal::media_de_rendas(&rendas)?,
                fluxos,
            };

            Ok((estrategia, resultados_de_estrategia))
        })
        .collect::<Result<_>>()?;

    Ok(resultados)
}

fn testar_estrategias_para_fluxo<'a>(
    inflacao: &Inflacao,
    titulo: &'a Titulo,
    dia_inicio: NaiveDate,
    valor_inicio: Decimal<2>,
    periodo: Duration,
    carencia: Duration,
    estrategias: &[Box<dyn Estrategia + '_>],
) -> Result<HashMap<String, ResultadoDeEstrategia<'a>>> {
    let mut fluxo = FluxoInvestimento::new(titulo);
    fluxo
        .comprar_a_partir_de(dia_inicio, QuantidadeOuValor::Valor(valor_inicio))
        .context("compra inicial precisa ser possível")?;

    let resultados = estrategias
        .iter()
        .filter_map(|estrategia| {
            let fluxo = estrategia
                .aplicar(fluxo.clone(), dia_inicio + carencia, periodo - carencia)
                .ok()?;
            let renda = RendaReal::new(
                inflacao,
                &fluxo,
                dia_inicio + carencia,
                dia_inicio + periodo,
            );

            Some((estrategia.nome(), ResultadoDeEstrategia { fluxo, renda }))
        })
        .collect();

    Ok(resultados)
}
