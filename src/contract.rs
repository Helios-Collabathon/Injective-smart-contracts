use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::persona::Persona;
use crate::state::{PERSONAS, PERSONA_LOOKUP};
use crate::wallet::Wallet;
use crate::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

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
        ExecuteMsg::RemoveWallet { wallet } => remove_wallet(_deps, _env, _info, wallet),
    }
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

    let mut persona = PERSONAS
        .load(deps.storage, info.sender.clone())
        .unwrap_or(Persona::new(vec![]));

    if persona.get_linked_wallets().contains(&wallet) {
        return Err(ContractError::CannotAddAddressBecauseItIsAlreadyLinked);
    }

    persona.add_wallet(wallet.clone());

    PERSONAS.save(deps.storage, info.sender.clone(), &persona)?;

    let mut addresses = PERSONA_LOOKUP
        .load(deps.storage, wallet.clone().get_id())
        .unwrap_or_default();

    addresses.push(info.sender.clone());

    PERSONA_LOOKUP.save(deps.storage, wallet.clone().get_id(), &addresses)?;

    Ok(Response::new()
        .add_attribute("action", "add_persona")
        .add_attribute("persona", persona.to_json()))
}

fn remove_wallet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wallet: Wallet,
) -> Result<Response, ContractError> {
    let mut persona = PERSONAS
        .load(deps.storage, info.sender.clone())
        .unwrap_or(Persona::new(vec![]));

    persona.remove_wallet(wallet.clone());

    let mut addresses = PERSONA_LOOKUP
        .load(deps.storage, wallet.clone().get_id())
        .unwrap_or_default();

    if addresses.contains(&info.sender.clone()) {
        addresses.retain(|x| !x.eq(&info.sender.clone()));
    }

    PERSONA_LOOKUP.save(deps.storage, wallet.clone().get_id(), &addresses)?;
    PERSONAS.save(deps.storage, info.sender.clone(), &persona)?;

    Ok(Response::new()
        .add_attribute("action", "remove_persona")
        .add_attribute("persona", persona.to_json()))
}
