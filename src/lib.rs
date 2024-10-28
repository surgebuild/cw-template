pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

// Import the correct functions from your `contract` module
use crate::contract::{instantiate as constant_instantiate, execute as contract_execute, query as contract_query};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Directly use the `instantiate` function from your `contract` module
    constant_instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Directly use the `execute` function from your `contract` module
    contract_execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    // Use the renamed `contract_query` function
    contract_query(deps, env, msg)
}
