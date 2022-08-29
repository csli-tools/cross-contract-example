#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, Uint64, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, CONFIG, NUM_RESERVATIONS};

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
        denom: msg.denom.clone(),
    };
    let amount = Uint64::zero();

    CONFIG.save(deps.storage, &config)?;
    NUM_RESERVATIONS.save(deps.storage, &amount)?;

    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner)
        .add_attribute("denom", msg.denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterWithPayment { dinner_contract } => {
            execute_register_with_payment(info, dinner_contract)
        }
        ExecuteMsg::RegisterWithScholarship { dinner_contract } => {
            execute_register_with_scholarship(info, dinner_contract)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_REGISTER_WITH_PAYMENT => reply_register_with_payment(deps, msg),
        REPLY_REGISTER_WITH_SCHOLARSHIP => reply_register_with_scholarship(deps, msg),
        id => Err(ContractError::UnknownReplyID { id }),
    }
}

pub fn execute_register_with_payment(
    info: MessageInfo,
    dinner_contract: String,
) -> Result<Response, ContractError> {
    let action = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: dinner_contract,
        msg: to_binary(
            &cross_contract_dinner::msg::ExecuteMsg::RegisterWithPayment {
                address: info.sender,
            },
        )
        .unwrap(),
        funds: info.funds,
    });
    let sub_msg: SubMsg = SubMsg::reply_always(action, REPLY_REGISTER_WITH_PAYMENT);
    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "execute_register_with_payment")
        .add_submessage(sub_msg))
}

pub fn execute_register_with_scholarship(
    info: MessageInfo,
    dinner_contract: String,
) -> Result<Response, ContractError> {
    let action = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: dinner_contract,
        msg: to_binary(
            &cross_contract_dinner::msg::ExecuteMsg::RegisterWithScholarship {
                address: info.sender,
            },
        )
        .unwrap(),
        funds: vec![],
    });
    let sub_msg: SubMsg = SubMsg::reply_always(action, REPLY_REGISTER_WITH_SCHOLARSHIP);
    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "execute_register_with_scholarship")
        .add_submessage(sub_msg))
}

fn reply_register_with_payment(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let Err(err) = msg.result.into_result() {
        return Err(ContractError::ReplyError {
            code: REPLY_REGISTER_WITH_PAYMENT,
            msg: err,
        });
    };
    NUM_RESERVATIONS.update(deps.storage, |num: Uint64| -> Result<_, ContractError> {
        Ok(num.saturating_add(1u64.into()))
    })?;
    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "reply")
        .add_attribute("reply_id", REPLY_REGISTER_WITH_PAYMENT.to_string()))
}

fn reply_register_with_scholarship(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let Err(err) = msg.result.into_result() {
        return Err(ContractError::ReplyError {
            code: REPLY_REGISTER_WITH_SCHOLARSHIP,
            msg: err,
        });
    };
    NUM_RESERVATIONS.update(deps.storage, |num: Uint64| -> Result<_, ContractError> {
        Ok(num.saturating_add(1u64.into()))
    })?;
    Ok(Response::new()
        .add_attribute("contract", "demo-totals")
        .add_attribute("method", "reply")
        .add_attribute("reply_id", REPLY_REGISTER_WITH_SCHOLARSHIP.to_string()))
}
