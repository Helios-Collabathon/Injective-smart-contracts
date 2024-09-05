use crate::{persona::Persona, wallet::Wallet};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddWallet { wallet: Wallet },
    RemoveWallet { wallet: Wallet },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Persona)]
    GetPersona { address: Addr },

    #[returns(Vec<Addr>)]
    GetPersonaFromLinkedWallet { wallet: Wallet },
}
