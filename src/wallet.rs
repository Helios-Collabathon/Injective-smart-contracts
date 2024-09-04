use crate::chain::Chain;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
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
