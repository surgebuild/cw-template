#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, save_state, load_state};

// Use `#[entry_point]` for each of the functions if you want them to be accessible from outside.
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        stored_string: msg.initial_string,
    };
    save_state(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateString { new_string } => try_update_string(deps, new_string),
    }
}


pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetString {} => query_stored_string(deps),
    }
}

fn try_update_string(deps: DepsMut, new_string: String) -> Result<Response, ContractError> {
    let mut state = load_state(deps.storage)?;
    state.stored_string = new_string;
    save_state(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "try_update_string"))
}

fn query_stored_string(deps: Deps) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    to_json_binary(&state.stored_string)
}
