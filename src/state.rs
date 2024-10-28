use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;

// Define the structure to hold the string
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub stored_string: String,
}

// Storage key
pub const STATE: Item<State> = Item::new("state");

// Helper function to save the state
pub fn save_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    STATE.save(storage, state)
}

// Helper function to load the state
pub fn load_state(storage: &dyn Storage) -> StdResult<State> {
    STATE.load(storage)
}
