use crate::decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub enum QuantidadeOuValor {
    Quantidade(Decimal<2>),
    Valor(Decimal<2>),
}

impl QuantidadeOuValor {
    pub fn quantidade_e_valor<const N: u8>(self, preco: Decimal<N>) -> (Decimal<2>, Decimal<2>) {
        match self {
            QuantidadeOuValor::Quantidade(quantidade) => (quantidade, quantidade * preco),
            QuantidadeOuValor::Valor(valor) => {
                let quantidade = valor / preco;

                (quantidade, quantidade * preco)
            }
        }
    }
}
