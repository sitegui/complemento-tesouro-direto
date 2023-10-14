use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerieTemporal<T> {
    valores: Vec<(NaiveDate, T)>,
}

impl<T: Copy> SerieTemporal<T> {
    pub fn new(mut valores: Vec<(NaiveDate, T)>) -> Self {
        valores.sort_by_key(|&(dia, _)| dia);
        SerieTemporal { valores }
    }

    pub fn primeiro(&self) -> Option<(NaiveDate, T)> {
        self.valores.first().copied()
    }

    pub fn ultimo(&self) -> Option<(NaiveDate, T)> {
        self.valores.last().copied()
    }

    pub fn valor_atual(&self, dia: NaiveDate) -> Option<T> {
        match self.valores.binary_search_by_key(&dia, |&(dia, _)| dia) {
            Ok(index) => Some(self.valores[index].1),
            Err(index) => index.checked_sub(1).map(|index| self.valores[index].1),
        }
    }

    pub fn iter_comecando_em(
        &self,
        minimo: NaiveDate,
    ) -> impl Iterator<Item = (NaiveDate, T)> + '_ {
        let index = match self.valores.binary_search_by_key(&minimo, |&(dia, _)| dia) {
            Ok(index) | Err(index) => index,
        };

        self.valores.iter().skip(index).copied()
    }
}
