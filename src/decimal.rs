use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub struct Decimal<const N: u8>(i64);

impl<const N: u8> Decimal<N> {
    pub fn zero() -> Self {
        Decimal(0)
    }
}

impl<'de, const N: u8> Deserialize<'de> for Decimal<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let texto = String::deserialize(deserializer)?;

        let (inteiro, decimais) = texto
            .split_once(',')
            .ok_or_else(|| Error::custom("faltando a vírgula"))?;

        if decimais.len() != N as usize {
            return Err(Error::custom(format!(
                "número '{}' não tem {} casas decimais",
                decimais, N
            )));
        }

        let inteiro: i64 = inteiro.parse().map_err(Error::custom)?;
        let decimais: i64 = decimais.parse().map_err(Error::custom)?;

        Ok(Decimal(inteiro * 10i64.pow(N as u32) + decimais))
    }
}
