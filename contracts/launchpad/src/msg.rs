use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

// use crate::state::{StakerInfo, StakerResponse, StakerListResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub raising_denom: String,
    pub offering_token: Addr,
    pub start_time: u64,
    pub end_time: u64,
    pub raising_amount: Uint128,
    pub offering_amount: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {
    pub msg: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        amount: Uint128,
    },
    Harvest {},
    UpdateConfig {
        raising_denom: Option<String>,
        offering_token: Option<Addr>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        raising_amount: Option<Uint128>,
        offering_amount: Option<Uint128>,
    },
    FinalWithdraw {
        raise_amount: Uint128, // amount of raising token to withdraw
        offer_amount: Uint128, // amount of tokens that are being sold to withdraw
    },
    FlipAllowClaim {},
}

#[cw_serde]
pub enum QueryAnswer {
    // Define your query answers here...
}

// Define query structs for the responses
#[cw_serde]
pub struct UserInfoResponse {
    pub address: Addr,
    pub amount: Uint128,
    pub claimed: bool,
}

#[cw_serde]
pub struct TotalAmountResponse {
    pub total_amount: Uint128,
}

#[cw_serde]
pub struct ConfigResponse {
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
pub struct IsClaimingAllowedResponse {
    pub is_claiming_allowed: bool,
}

// Define the query message enum
#[cw_serde]
pub enum QueryMsg {
    GetUser { address: Addr },
    GetTotalAmount {},
    Config {},
    IsClaimingAllowed {},
}
