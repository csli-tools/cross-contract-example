use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("ERR_STD|{0}")]
    Std(#[from] StdError),

    #[error("ERR_NO_SCHOLARSHIP|Sender not in scholarship list and must pay")]
    Unauthorized {},

    #[error("Address is already registered for the dinner")]
    AlreadyRegistered {},

    #[error("Must attach at least 0.01 ujunox (or another native denom)")]
    MustAttachFunds {},

    #[error("{code:?}|{msg:?}")]
    CustomError { code: String, msg: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
