use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("ERR_STD|{0}")]
    Std(#[from] StdError),

    #[error("ERR_NO_SCHOLARSHIP|Sender not in scholarship list and must pay")]
    Unauthorized {},

    #[error("ERR_UNKNOWN_REPLY|Unknown reply ID")]
    UnknownReplyID {},

    #[error("ERR_REPLY_ERROR|Unknown reply ID")]
    ReplyError { code: String, msg: String },

    #[error("{code:?}|{msg:?}")]
    CustomError { code: String, msg: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror for details.
}
