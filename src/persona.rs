use crate::wallet::Wallet;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Persona {
    linked_wallets: Vec<Wallet>,
}

impl Persona {
    pub fn new(linked_wallets: Vec<Wallet>) -> Self {
        Persona { linked_wallets }
    }

    pub fn add_wallet(&mut self, wallet: Wallet) {
        self.linked_wallets.push(wallet)
    }

    pub(crate) fn to_json(&self) -> impl Into<String> {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
