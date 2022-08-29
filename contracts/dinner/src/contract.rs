#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, has_coins, to_binary, Addr, Api, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, DINNER_REGISTRANTS, SCHOLARSHIPS_ADDRESS};

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:cw-cross-contract-calls-dinner";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The scholarship list is set during instantiation
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
        owner,
        denom: msg.denom.clone(),
    };
    let scholarship_list: Vec<Addr> = vec![];
    let scholarship_address = deps
        .api
        .addr_validate(msg.scholarship_address.as_str())
        .expect("Wrong scholarship address");

    CONFIG.save(deps.storage, &config)?;
    DINNER_REGISTRANTS.save(deps.storage, &scholarship_list)?;
    SCHOLARSHIPS_ADDRESS.save(deps.storage, &scholarship_address)?;

    Ok(Response::new()
        .add_attribute("contract", "dinner")
        .add_attribute("method", "instantiate"))
}

// Taken from:
// https://github.com/CosmWasm/cw-plus/blame/fc089febdab836400982eb096a545997a2bf4aed/contracts/cw1-whitelist/src/contract.rs#L38-L40
pub fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterWithScholarship { address } => register_with_scholarship(deps, address),
        ExecuteMsg::RegisterWithPayment { address } => register_with_payment(deps, info, address),
    }
}

pub fn register_with_payment(
    deps: DepsMut,
    info: MessageInfo,
    address: Addr,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !has_coins(info.funds.as_slice(), &coin(10000, config.denom)) {
        return Err(ContractError::MustAttachFunds {});
    }
    let mut registrants = DINNER_REGISTRANTS.load(deps.storage)?;
    if registrants.iter().any(|registered| registered == &address) {
        return Err(ContractError::AlreadyRegistered {});
    };
    registrants.push(address);
    DINNER_REGISTRANTS.save(deps.storage, &registrants)?;
    Ok(Response::new()
        .add_attribute("contract", "dinner")
        .add_attribute("method", "register_with_payment"))
}

pub fn register_with_scholarship(deps: DepsMut, address: Addr) -> Result<Response, ContractError> {
    let mut registrants = DINNER_REGISTRANTS.load(deps.storage)?;
    if registrants.iter().any(|registered| registered == &address) {
        return Err(ContractError::AlreadyRegistered {});
    };
    let whitelist: cw1_whitelist::msg::AdminListResponse = deps.querier.query_wasm_smart(
        SCHOLARSHIPS_ADDRESS.load(deps.storage)?,
        &cw1_whitelist::msg::QueryMsg::AdminList::<String> {},
    )?;
    if !whitelist
        .admins
        .iter()
        .any(|addr| addr == &address.clone().into_string())
    {
        return Err(ContractError::Unauthorized {});
    }
    registrants.push(address);
    DINNER_REGISTRANTS.save(deps.storage, &registrants)?;
    Ok(Response::new()
        .add_attribute("contract", "dinner")
        .add_attribute("method", "register_with_scholarship"))
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
    let all_registrants = cfg.into_iter().collect();
    Ok(all_registrants)
}

pub fn query_is_address_registered(deps: Deps, address: Addr) -> StdResult<bool> {
    let registrants = DINNER_REGISTRANTS.load(deps.storage)?;
    let is_registered = registrants.iter().any(|a| a.as_ref() == address);
    Ok(is_registered)
}
