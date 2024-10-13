use std::ops::Mul;

use cosmwasm_std::{
    attr, coins, entry_point, to_json_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use cw20_base::ContractError;
use dojoswap::util::migrate_version;

use crate::{
    msg::{
        ConfigResponse, ExecuteMsg, InstantiateMsg, IsClaimingAllowedResponse, MigrateMsg,
        QueryMsg, TotalAmountResponse, UserInfoResponse,
    },
    state::{State, STATE, USER_INFO},
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:launchpad";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        admin: msg.admin,
        raising_denom: msg.raising_denom.clone(),
        offering_token: msg.offering_token,
        start_time: msg.start_time,
        end_time: msg.end_time,
        raising_amount: msg.raising_amount,
        offering_amount: msg.offering_amount,
        total_amount: Uint128::zero(),
        allow_claim: false,
    };

    if msg.start_time > msg.end_time {
        return Err(StdError::generic_err("Start time must be before end time"));
    }

    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Deposit { amount } => deposit(deps, env, info, amount),
        ExecuteMsg::Harvest {} => harvest(deps, env, info),
        ExecuteMsg::UpdateConfig {
            raising_denom,
            offering_token,
            start_time,
            end_time,
            raising_amount,
            offering_amount,
        } => update_config(
            deps,
            env,
            info,
            raising_denom,
            offering_token,
            start_time,
            end_time,
            raising_amount,
            offering_amount,
        ),
        ExecuteMsg::FinalWithdraw {
            raise_amount,
            offer_amount,
        } => final_withdraw(deps, env, info, raise_amount, offer_amount),
        ExecuteMsg::FlipAllowClaim {} => flip_allow_claim(deps, info),
    }
}

const EXPECTED_PREVIOUS_CONTRACT_VERSION: &str = "0.1.2";
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_version(
        deps,
        EXPECTED_PREVIOUS_CONTRACT_VERSION,
        CONTRACT_NAME,
        CONTRACT_VERSION,
    )?;

    Ok(Response::default())
}

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    raising_denom: Option<String>,
    offering_token: Option<Addr>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    raising_amount: Option<Uint128>,
    offering_amount: Option<Uint128>,
) -> StdResult<Response> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != deps.api.addr_canonicalize(state.admin.as_str())? {
        return Err(StdError::generic_err("Unauthorized: not admin"));
    }

    if state.total_amount != Uint128::zero() {
        return Err(StdError::generic_err(
            "Unauthorized: launchpad already in progress",
        ));
    }

    if start_time > end_time {
        return Err(StdError::generic_err("Start time must be before end time"));
    }

    if raising_denom.is_some() {
        state.raising_denom = raising_denom.unwrap();
    }

    if offering_token.is_some() {
        state.offering_token = offering_token.unwrap();
    }

    if offering_amount.is_some() {
        state.offering_amount = offering_amount.unwrap();
    }

    if start_time.is_some() {
        state.start_time = start_time.unwrap();
    }

    if end_time.is_some() {
        state.end_time = end_time.unwrap();
    }

    if raising_amount.is_some() {
        state.raising_amount = raising_amount.unwrap();
    }

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_config")]))
}

fn deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    // Check if it's within the launchpad time
    if env.block.time.seconds() < state.start_time || env.block.time.seconds() > state.end_time {
        return Err(StdError::generic_err("Not in launchpad time"));
    }

    // Ensure the deposit amount is greater than 0
    if amount.is_zero() {
        return Err(StdError::generic_err(
            "Deposit amount must be greater than 0",
        ));
    }

    // Transfer tokens from sender to the contract
    if info.funds.len() != 1 || info.funds[0].denom != state.raising_denom {
        return Err(StdError::generic_err("Wrong denom"));
    }

    if info.funds[0].amount != amount {
        return Err(StdError::generic_err("Wrong amount"));
    }

    // Update user information
    let mut user = USER_INFO.load(deps.storage, info.sender.to_string())?;
    // if user.amount.is_zero() {
    //     // Add the user to the addressList
    //     state.address_list.push(env.message.sender.clone());
    // }

    user.amount += amount;
    USER_INFO.save(deps.storage, info.sender.to_string(), &user)?;

    // Update total amount
    state.total_amount += amount;
    STATE.save(deps.storage, &state)?;

    return Ok(Response::new().add_attributes(vec![
        attr("action", "deposit"),
        attr("address", info.sender.to_string()),
        attr("amount", amount),
    ]));
}

fn harvest(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, StdError> {
    let state = STATE.load(deps.storage)?;

    // Check if it's after the harvest time
    if env.block.time.seconds() <= state.end_time {
        return Err(StdError::generic_err("Not in harvest time"));
    }

    // Get user information
    let mut user = USER_INFO.load(deps.storage, info.sender.to_string())?;

    // Check if the user has participated
    if user.amount.is_zero() {
        return Err(StdError::generic_err("User has not participated"));
    }

    // Check if the user has already claimed
    if user.claimed {
        return Err(StdError::generic_err("Already claimed"));
    }

    // Check if claiming is allowed
    if !state.allow_claim {
        return Err(StdError::generic_err("Claiming not allowed"));
    }

    // Calculate offering and refund amounts
    // 1e12 & 1e6
    let user_allocation = user
        .amount
        .mul(Uint128::new(1000000000000))
        .checked_div(state.total_amount)?
        .checked_div(Uint128::new(1000000))?;
    let offering_amount = get_offering_amount(
        user_allocation,
        user.amount,
        state.total_amount,
        state.raising_amount,
        state.offering_amount,
    )?;
    let refund_amount = get_refunding_amount(
        user_allocation,
        user.amount,
        state.total_amount,
        state.raising_amount,
        state.offering_amount,
    )?;

    let mut messages: Vec<CosmosMsg> = vec![];
    // Transfer offering tokens
    if offering_amount > Uint128::from(0u128) {
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.offering_token.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: offering_amount,
            })?,
            funds: vec![],
        }));
    }

    // Transfer refund tokens
    if refund_amount > Uint128::from(0u128) {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(refund_amount.into(), state.raising_denom),
        }));
    }

    // Update user information
    user.claimed = true;
    USER_INFO.save(deps.storage, info.sender.to_string(), &user)?;

    return Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("action", "harvest"),
        attr("address", info.sender.to_string()),
        attr("offering_amount", offering_amount),
        attr("refund_amount", refund_amount),
    ]));
}

pub fn final_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    raise_amount: Uint128,
    offer_amount: Uint128,
) -> Result<Response, StdError> {
    let state = STATE.load(deps.storage)?;
    let raising_denom = state.clone().raising_denom.to_string();

    // Check if the sender is the admin
    
    // if info.sender.to_string() != state.admin.to_string() {
    if deps.api.addr_canonicalize(info.sender.as_str())? != deps.api.addr_canonicalize(state.admin.as_str())? {
        return Err(StdError::generic_err("Unauthorized: not admin"));
    }

    let raising_balance = deps
        .querier
        .query_balance(env.contract.address.to_string(), state.raising_denom)?;

    let msg = cw20::Cw20QueryMsg::Balance {
        address: env.contract.address.to_string(),
    };
    let balance_response: cw20::BalanceResponse = deps
        .querier
        .query_wasm_smart(&state.offering_token, &msg)
        .unwrap();

    // Check if the requested raise_amount is available
    if raise_amount > raising_balance.amount {
        return Err(StdError::generic_err("Not enough raising tokens"));
    }

    // Check if the requested offer_amount is available
    if offer_amount > balance_response.balance {
        return Err(StdError::generic_err("Not enough offering tokens"));
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    // Transfer raising denom tokens to admin
    if raise_amount > Uint128::zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(raise_amount.into(), raising_denom),
        }));
    }

    // Transfer offering tokens to admin
    if offer_amount > Uint128::zero() {
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.offering_token.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: offer_amount,
            })?,
            funds: vec![],
        }));
    }

    return Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("action", "final_withdraw"),
        attr("offer_amount", offer_amount),
        attr("raise_amount", raise_amount),
    ]));
}

pub fn flip_allow_claim(deps: DepsMut, info: MessageInfo) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != deps.api.addr_canonicalize(state.admin.as_str())? {
        return Err(StdError::generic_err("Unauthorized: not admin"));
    }

    state.allow_claim = !state.allow_claim;
    STATE.save(deps.storage, &state)?;

    return Ok(Response::default());
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUser { address } => to_json_binary(&query_user(deps, env, address)?),
        QueryMsg::GetTotalAmount {} => to_json_binary(&query_total_amount(deps)?),
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::IsClaimingAllowed {} => to_json_binary(&query_is_claiming_allowed(deps)?),
    }
}

fn query_user(deps: Deps, _env: Env, address: Addr) -> StdResult<UserInfoResponse> {
    let user = USER_INFO.load(deps.storage, address.to_string())?;

    Ok(UserInfoResponse {
        address,
        amount: user.amount,
        claimed: user.claimed,
    })
}

fn query_total_amount(deps: Deps) -> StdResult<TotalAmountResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(TotalAmountResponse {
        total_amount: state.total_amount,
    })
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(ConfigResponse {
        admin: state.admin,
        raising_denom: state.raising_denom.clone(),
        offering_token: state.offering_token,
        start_time: state.start_time,
        end_time: state.end_time,
        raising_amount: state.raising_amount,
        offering_amount: state.offering_amount,
        total_amount: state.total_amount,
        allow_claim: state.allow_claim,
    })
}

fn query_is_claiming_allowed(deps: Deps) -> StdResult<IsClaimingAllowedResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(IsClaimingAllowedResponse {
        is_claiming_allowed: state.allow_claim,
    })
}

// Helper functions for calculating amounts
fn get_offering_amount(
    user_allocation: Uint128,
    user_amount: Uint128,
    total_amount: Uint128,
    raising_amount: Uint128,
    offering_amount: Uint128,
) -> StdResult<Uint128> {
    // if (totalAmount > raisingAmount) {
    //     uint256 allocation = getUserAllocation(_user);
    //     return offeringAmount.mul(allocation).div(1e6);
    //   }
    //   else {
    //     // userInfo[_user] / (raisingAmount / offeringAmount)
    //     return userInfo[_user].amount.mul(offeringAmount).div(raisingAmount);
    //   }

    // userInfo[_user].amount.mul(offeringAmount).div(raisingAmount);

    if total_amount > raising_amount {
        return Ok(offering_amount
            .mul(user_allocation)
            .checked_div(Uint128::new(1000000))?);
    } else {
        return Ok(user_amount
            .mul(offering_amount)
            .checked_div(raising_amount)?);
    }

    // if raising_amount > Uint128::zero() {
    //     let allocation = user_amount.multiply_ratio(Uint128::new(1, 1), raising_amount);
    //     allocation.multiply_ratio(offering_amount, Uint128::new(1, 1))
    // } else {
    //     user_amount.multiply_ratio(offering_amount, raising_amount)
    // }
}

fn get_refunding_amount(
    user_allocation: Uint128,
    user_amount: Uint128,
    total_amount: Uint128,
    raising_amount: Uint128,
    _offering_amount: Uint128,
) -> StdResult<Uint128> {
    // if (totalAmount <= raisingAmount) {
    //     return 0;
    //   }
    //   uint256 allocation = getUserAllocation(_user);
    //   uint256 payAmount = raisingAmount.mul(allocation).div(1e6);
    //   return userInfo[_user].amount.sub(payAmount);

    if total_amount <= raising_amount {
        return Ok(Uint128::zero());
    }
    let pay_amount = raising_amount
        .mul(user_allocation)
        .checked_div(Uint128::new(1000000))?;
    let result = user_amount.checked_sub(pay_amount)?;

    // if raising_amount > Uint128::zero() {
    //     let allocation = user_amount.multiply_ratio(Uint128::new(1, 1), raising_amount);
    //     user_amount - allocation
    // } else {
    //     Uint128::zero()
    // }
    return Ok(result);
}
