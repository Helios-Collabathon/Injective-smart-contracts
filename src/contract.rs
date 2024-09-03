use crate::chain::Chain;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::persona::Persona;
use crate::state::PERSONAS;
use crate::wallet::Wallet;
use crate::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

const CONTRACT_NAME: &str = "crates.io:inj-interchain-persona";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handles contract instantiation, initializing necessary storage.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Return success response with attributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION))
}

/// Executes contract functions based on incoming messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddWallet { wallet } => add_wallet(_deps, _env, _info, wallet),
        ExecuteMsg::RemoveWallet {} => remove_wallet(_deps, _env, _info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

fn add_wallet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wallet: Wallet,
) -> Result<Response, ContractError> {
    if info
        .sender
        .clone()
        .to_string()
        .eq(&wallet.clone().get_address())
    {
        return Err(ContractError::CannotAddOwnAddressAsLinkedAddress);
    }

    if wallet.clone().get_chain() == Chain::Injective {
        let adrr = deps
            .api
            .addr_validate(wallet.clone().get_address().as_str())?;

        let found = PERSONAS.load(deps.storage, adrr).is_ok();
        if found {
            return Err(ContractError::CannotAddAddressBecauseItHasAPersona);
        }
    }

    let mut persona = PERSONAS
        .load(deps.storage, info.sender.clone())
        .unwrap_or(Persona::new(vec![]));

    persona.add_wallet(wallet.clone());

    PERSONAS.save(deps.storage, info.sender.clone(), &persona)?;

    Ok(Response::new()
        .add_attribute("action", "add_persona")
        .add_attribute("persona", persona.to_json()))
}

fn remove_wallet(_deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    todo!()
}
