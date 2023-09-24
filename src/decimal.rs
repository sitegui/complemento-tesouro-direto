use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct Decimal<const N: u8>(i64);

impl<const N: u8> Decimal<N> {
    pub fn zero() -> Self {
        Decimal(0)
    }

    pub fn new(valor: f32) -> Self {
        let inteiro = (valor * 10f32.powi(N as i32)).round();

        Decimal(inteiro as i64)
    }
}
