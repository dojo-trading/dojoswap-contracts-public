use cosmwasm_std::{to_json_binary, Addr, QuerierWrapper, QueryRequest, StdResult, WasmQuery};
use cw20::{Cw20QueryMsg, MinterResponse};

/// Query asset price igonoring price age
pub fn query_anc_minter(querier: &QuerierWrapper, token: Addr) -> StdResult<String> {
    let res: MinterResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: token.to_string(),
        msg: to_json_binary(&Cw20QueryMsg::Minter {})?,
    }))?;

    Ok(res.minter)
}
