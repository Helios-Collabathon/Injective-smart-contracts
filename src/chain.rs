use std::fmt;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Chain {
    Injective,
    MultiversX,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chain::Injective => { write!(f, "injective") }
            Chain::MultiversX => { write!(f, "multivers_x") }
        }
    }
}
