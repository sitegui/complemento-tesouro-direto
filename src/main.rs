use crate::decimal::Decimal;
use crate::estrategias::venda_continua::venda_continua;
use crate::inflacao::Inflacao;
use crate::ler_titulos::ler_titulos;
use crate::titulo::Titulo;
use chrono::{Duration, NaiveDate};

mod decimal;
mod estrategias;
mod fluxo_investimento;
mod inflacao;
mod ler_titulos;
mod quantidade_ou_valor;
mod serie_temporal;
mod titulo;

fn main() {
    let tipo = "Tesouro IPCA+ com Juros Semestrais";
    let tempo_minimo = Duration::days(365 * 10);
    let valor_inicio = Decimal::<2>::new(100_000.0);
    let num_meses = 12 * 10;

    let inflacao = Inflacao::baixar_com_cache();
    let titulos = ler_titulos(tipo);

    let titulos = titulos_com_dados_suficientes(&titulos, tempo_minimo);

    for titulo in titulos {
        println!(
            "{}: {} a {}",
            titulo.vencimento, titulo.inicio_dado, titulo.fim_dado
        );

        let fluxo = venda_continua(titulo, titulo.inicio_dado, valor_inicio, num_meses);
        for evento in fluxo.eventos() {
            eprintln!("evento = {:?}", evento);
        }
    }

    println!(
        "{}",
        inflacao.corrigir(
            Decimal::<2>::new(100.0),
            NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()
        )
    );
}

fn titulos_com_dados_suficientes(titulos: &[Titulo], tempo_minimo: Duration) -> Vec<&Titulo> {
    titulos
        .iter()
        .filter(|titulo| titulo.fim_dado - titulo.inicio_dado > tempo_minimo)
        .collect()
}
