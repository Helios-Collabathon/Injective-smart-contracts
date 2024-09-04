use crate::wallet::Wallet;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddWallet { wallet: Wallet },
    RemoveWallet { wallet: Wallet },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
