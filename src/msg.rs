use crate::wallet::Wallet;
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
    #[returns(crate::persona::Persona)]
    GetPersona { address: Addr },

    #[returns(Vec<Addr>)]
    GetPersonaFromLinkedWallet { wallet: Wallet },
}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::MigrateFns))]
pub enum MigrateMsg {}
