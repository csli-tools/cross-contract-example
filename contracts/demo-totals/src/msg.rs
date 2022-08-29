use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Most folks with pay a little to register for their dinner reservation.
    RegisterWithPayment { dinner_contract: String },
    // Some folks have a scholarship and don't have to pay.
    // They will be on the list stored in the whitelist contract.
    RegisterWithScholarship { dinner_contract: String },
}
