use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct State {
    pub admin: Addr,
    pub raising_denom: String,
    pub offering_token: Addr,
    pub start_time: u64,
    pub end_time: u64,
    pub raising_amount: Uint128,
    pub offering_amount: Uint128,
    pub total_amount: Uint128,
    pub allow_claim: bool,
}

#[cw_serde]
pub struct User {
    pub amount: Uint128,
    pub claimed: bool,
}

pub const STATE_KEY: &str = "state";
pub const STATE: Item<State> = Item::new(STATE_KEY);
pub const USER_INFO: Map<String, User> = Map::new("user_info");
