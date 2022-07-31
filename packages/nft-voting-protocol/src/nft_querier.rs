use cosmwasm_std::{QuerierWrapper, StdResult};
use cw721::{Cw721QueryMsg, OwnerOfResponse, TokensResponse};

// Query CW721
pub fn query_token_owner(
    querier: &QuerierWrapper,
    contract_addr: &String,
    token_id: &String,
) -> StdResult<String> {
    let res: OwnerOfResponse = querier.query_wasm_smart(
        contract_addr,
        &Cw721QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        },
    )?;

    Ok(res.owner)
}

// Query CW721
pub fn query_has_tokens(
    querier: &QuerierWrapper,
    contract_addr: &String,
    owner: &String,
) -> StdResult<bool> {
    let res: TokensResponse = querier.query_wasm_smart(
        contract_addr,
        &Cw721QueryMsg::Tokens {
            owner: owner.clone(),
            start_after: None,
            limit: Some(1),
        },
    )?;

    Ok(!res.tokens.is_empty())
}
