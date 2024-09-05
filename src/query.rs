use crate::{
    msg::QueryMsg,
    persona::Persona,
    state::{PERSONAS, PERSONA_LOOKUP},
    wallet::Wallet,
};
use cosmwasm_std::{entry_point, to_json_binary, Addr, Binary, Deps, Env, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPersona { address } => to_json_binary(&get_persona(deps, address)?),
        QueryMsg::GetPersonaFromLinkedWallet { wallet } => {
            to_json_binary(&get_persona_by_linked_wallet(deps, wallet)?)
        }
    }
}

fn get_persona(deps: Deps, address: Addr) -> StdResult<Persona> {
    Ok(PERSONAS
        .load(deps.storage, address)
        .unwrap_or(Persona::new(vec![])))
}

fn get_persona_by_linked_wallet(deps: Deps, wallet: Wallet) -> StdResult<Vec<Persona>> {
    let addresses = PERSONA_LOOKUP
        .load(deps.storage, wallet.get_id())
        .unwrap_or_default();
    let mut personas = Vec::new();
    for address in addresses {
        let persona = PERSONAS.load(deps.storage, address);
        if let Ok(persona) = persona {
            personas.push(persona);
        }
    }
    Ok(personas)
}
