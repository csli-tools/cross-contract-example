#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Reply, Response, StdResult,
    Uint64,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_execute_data;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, CONFIG};

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:cw-cross-contract-calls-reservation";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Reply IDs
const REPLY_REGISTER_WITH_PAYMENT: u64 = 0;
const REPLY_REGISTER_WITH_SCHOLARSHIP: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config {
        owner: owner.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterWithPayment { dinner_contract } => {
            execute_register_with_payment(deps, env, info, dinner_contract)
        } // ExecuteMsg::RegisterWithScholarship { dinner_contract } => {
          //     execute_register_with_scholarship(deps, env, info, dinner_contract)
          // }
    }
}

pub fn execute_register_with_payment(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    dinner_contract: Addr,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    println!("aloha config {:?}", config);
    let count = Uint64::from(19u64);
    let data_response = format!("Performed {} reservations.", count).into_bytes();
    Ok(Response::new()
        .add_attribute("method", "execute_register_with_payment")
        .set_data(data_response))
}

// pub fn execute_register_with_scholarship(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     dinner_contract: Addr,
// ) -> Result<Response, ContractError> {
//     let mut config: Config = CONFIG.load(deps.storage)?;
//     println!("aloha config {:?}", config);
//     let count = Uint64::from(19u64);
//     let data_response = format!("Performed {} reservations.", count).into_bytes();
//     Ok(Response::new()
//         .add_attribute("method", "execute_register_with_scholarship")
//         .set_data(data_response))
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_REGISTER_WITH_PAYMENT => {
            let res = parse_reply_execute_data(msg);
            if res.is_err() {
                return Err(ContractError::ReplyError {
                    code: "hardcoded error code REPLY_REGISTER_WITH_PAYMENT".to_string(),
                    msg: "hardcoded error msg REPLY_REGISTER_WITH_PAYMENT".to_string(),
                });
            }
            println!("REPLY_REGISTER_WITH_PAYMENT res {:?}", res);
            Ok(Response::new().add_attribute("reply", "register_with_payment"))
        }
        REPLY_REGISTER_WITH_SCHOLARSHIP => {
            let res = parse_reply_execute_data(msg);
            if res.is_err() {
                return Err(ContractError::ReplyError {
                    code: "hardcoded error code REPLY_REGISTER_WITH_SCHOLARSHIP".to_string(),
                    msg: "hardcoded error msg REPLY_REGISTER_WITH_SCHOLARSHIP".to_string(),
                });
            }
            println!("REPLY_REGISTER_WITH_SCHOLARSHIP res {:?}", res);
            Ok(Response::new().add_attribute("reply", "register_with_scholarship"))
        }
        _ => Err(ContractError::UnknownReplyID {}),
    }
}
