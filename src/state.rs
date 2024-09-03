use crate::persona::Persona;
use crate::wallet::Wallet;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const PERSONAS: Map<Addr, Persona> = Map::new("personas");

pub const PERSONA_LOOKUP: Map<Wallet, Addr> = Map::new("persona_lookup");
