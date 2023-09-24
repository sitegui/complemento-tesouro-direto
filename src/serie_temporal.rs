use chrono::NaiveDate;

#[derive(Debug, Clone)]
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
}
