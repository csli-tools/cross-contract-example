use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::Item;

// We might as well have one-letter keys like "s" or "d" to save ones and zeroes. Might as well.
// CONFIG will typically store values that might be updated later using a special function that checks if the sender is allowed to change it.
pub const SCHOLARSHIPS_ADDRESS: Item<Addr> = Item::new("s");
pub const DINNER_REGISTRANTS: Item<Vec<Addr>> = Item::new("d");
