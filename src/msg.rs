use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_string: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateString { new_string: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetStringResponse)]
    GetString {},
}

#[cw_serde]
pub struct GetStringResponse {
    pub stored_string: String,
}
