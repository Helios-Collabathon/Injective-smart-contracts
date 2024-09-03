use crate::chain::Chain;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Wallet {
    chain: Chain,
    address: String,
}

impl Wallet {
    pub fn new(chain: Chain, address: String) -> Self {
        Wallet { chain, address }
    }

    pub fn get_chain(self) -> Chain {
        self.chain
    }

    pub fn get_address(self) -> String {
        self.address
    }
}
