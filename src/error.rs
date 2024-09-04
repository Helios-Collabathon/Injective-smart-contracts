use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Cannot add own address as linked address")]
    CannotAddOwnAddressAsLinkedAddress,

    #[error("Cannot add address because it is already linked address")]
    CannotAddAddressBecauseItIsAlreadyLinked,

    #[error("Cannot delete address because persona does not exist")]
    CannotDeleteAddressBecausePersonaIsNotFound,

    #[error("Cannot delete address because persona does not have linked wallets")]
    CannotDeleteAddressBecausePersonaDoesNotHaveLinkedWallets,

    #[error("Cannot delete address because persona is not linked")]
    CannotDeleteAddressBecausePersonaIsNotLinked,
}
