use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub denom: String,
}

// We might as well have one-letter keys like "c" or "n" to save ones and zeroes. Might as well.
// CONFIG will typically store values that might be updated later using a special function that checks if the sender is allowed to change it.
pub const CONFIG: Item<Config> = Item::new("c");

// We might as well have one-letter keys like "s" or "d" to save ones and zeroes. Might as well.
pub const SCHOLARSHIPS_ADDRESS: Item<Addr> = Item::new("s");
pub const DINNER_REGISTRANTS: Item<Vec<Addr>> = Item::new("d");
