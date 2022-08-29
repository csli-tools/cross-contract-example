use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub denom: String,
}

// We might as well have one-letter keys like "c" or "n" to save ones and zeroes. Might as well.
// CONFIG will typically store values that might be updated later using a special function that checks if the sender is allowed to change it.
pub const CONFIG: Item<Config> = Item::new("c");
// Keep track of the total reservations made through this contract.
// We'll use this to demonstrate some behavior with Cosmos submessages and the reply pattern.
pub const NUM_RESERVATIONS: Item<Uint64> = Item::new("n");
