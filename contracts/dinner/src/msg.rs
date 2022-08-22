use cosmwasm_std::{Addr, CosmosMsg, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub scholarship_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Shows all addresses with dinner scholarships
    GetAllRegistrants {},
    /// Check if address is registered
    IsAddressRegistered { address: Addr },
}
