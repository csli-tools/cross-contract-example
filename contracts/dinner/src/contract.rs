#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Api, Binary, Deps, DepsMut, Env, MessageInfo, Order, Reply, Response,
    StdResult, Uint64,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_execute_data;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::state::{DINNER_REGISTRANTS, SCHOLARSHIPS_ADDRESS};

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:cw-cross-contract-calls-dinner";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The scholarship list is set during instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let scholarship_list: Vec<Addr> = map_validate(deps.api, &msg.scholarship_list)?;
    println!("aloha scholarship_list {:?}", scholarship_list);

    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Taken from:
// https://github.com/CosmWasm/cw-plus/blame/fc089febdab836400982eb096a545997a2bf4aed/contracts/cw1-whitelist/src/contract.rs#L38-L40
pub fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAllRegistrants {} => to_binary(&query_address_in_list(deps)?),
        QueryMsg::IsAddressRegistered { address } => {
            to_binary(&query_is_address_registered(deps, address)?)
        }
    }
}

pub fn query_address_in_list(deps: Deps) -> StdResult<Vec<Addr>> {
    let cfg = DINNER_REGISTRANTS.load(deps.storage)?;
    let all_registrants = cfg.into_iter().map(|a| a.into()).collect();
    Ok(all_registrants)
}

pub fn query_is_address_registered(deps: Deps, address: Addr) -> StdResult<bool> {
    let registrants = DINNER_REGISTRANTS.load(deps.storage)?;
    let is_registered = registrants.iter().any(|a| a.as_ref() == address);
    Ok(is_registered)
}
