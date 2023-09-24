use crate::inflacao::Inflacao;
use crate::ler_titulos::ler_titulos;
use crate::titulo::Titulo;
use chrono::Duration;

mod decimal;
mod inflacao;
mod ler_titulos;
mod serie_temporal;
mod titulo;

fn main() {
    let inflacao = Inflacao::baixar_com_cache();

    let titulos = ler_titulos("Tesouro IPCA+ com Juros Semestrais");
    let tempo_minimo = Duration::days(365 * 10);

    let titulos = titulos_com_dados_suficientes(&titulos, tempo_minimo);

    for titulo in titulos {
        println!(
            "{}: {} a {}",
            titulo.vencimento, titulo.inicio_dado, titulo.fim_dado
        );
    }
}

fn titulos_com_dados_suficientes(titulos: &[Titulo], tempo_minimo: Duration) -> Vec<&Titulo> {
    titulos
        .iter()
        .filter(|titulo| titulo.fim_dado - titulo.inicio_dado > tempo_minimo)
        .collect()
}
