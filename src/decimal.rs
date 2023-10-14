use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct Decimal<const N: u8>(i64);

impl<const N: u8> Decimal<N> {
    pub fn zero() -> Self {
        Decimal(0)
    }

    pub fn new(valor: f64) -> Self {
        let inteiro = (valor * 10f64.powi(N as i32)).round();

        Decimal(inteiro as i64)
    }

    pub fn as_float(self) -> f64 {
        self.0 as f64 / 10f64.powi(N as i32)
    }

    pub fn as_decimal<const M: u8>(self) -> Decimal<M> {
        Decimal::new(self.as_float())
    }
}

impl<const N: u8> Mul<f64> for Decimal<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Decimal((self.0 as f64 * rhs).round() as i64)
    }
}

impl<const N: u8, const M: u8> Mul<Decimal<M>> for Decimal<N> {
    type Output = Self;

    fn mul(self, rhs: Decimal<M>) -> Self::Output {
        self * rhs.as_float()
    }
}

impl<const N: u8> Div<f64> for Decimal<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Decimal((self.0 as f64 / rhs).floor() as i64)
    }
}

impl<const N: u8, const M: u8> Div<Decimal<M>> for Decimal<N> {
    type Output = Self;

    fn div(self, rhs: Decimal<M>) -> Self::Output {
        self / rhs.as_float()
    }
}

impl<const N: u8> DivAssign<f64> for Decimal<N> {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl<const N: u8> Add for Decimal<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Decimal(self.0 + rhs.0)
    }
}

impl<const N: u8> AddAssign for Decimal<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<const N: u8> Sub for Decimal<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0 - rhs.0)
    }
}

impl<const N: u8> SubAssign for Decimal<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<const N: u8> Display for Decimal<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*}", N as usize, self.as_float())
    }
}
