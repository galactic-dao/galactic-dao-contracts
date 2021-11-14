use cosmwasm_std::{QuerierWrapper, StdResult};
use cw721::{Cw721QueryMsg, OwnerOfResponse};

pub fn query_token_owner(
    querier: &QuerierWrapper,
    contract_addr: String,
    token_id: String,
) -> StdResult<String> {
    let res: OwnerOfResponse = querier.query_wasm_smart(
        contract_addr,
        &Cw721QueryMsg::OwnerOf {
            token_id,
            include_expired: None
        }
    )?;

    Ok(res.owner)
}
