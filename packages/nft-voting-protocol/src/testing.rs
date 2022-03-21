use std::collections::HashMap;

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Binary, Coin, ContractResult, Empty, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, WasmQuery,
};
use cw721::{Cw721QueryMsg, OwnerOfResponse};

/// Replacement of cosmwasm_std::testing::mock_dependencies
/// that allows for NFT ownership mocks
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockWasmQuerier> {
    let custom_querier: MockWasmQuerier =
        MockWasmQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

pub struct MockWasmQuerier {
    base: MockQuerier<Empty>,
    nft_querier: NftQuerier,
}

#[derive(Clone, Default)]
pub struct NftQuerier {
    // TokenId -> Owner Addr
    token_to_owner: HashMap<String, String>,
}

impl NftQuerier {
    pub fn new(token_to_owner: HashMap<String, String>) -> Self {
        NftQuerier { token_to_owner }
    }
}

impl Querier for MockWasmQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {:?}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl MockWasmQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                if contract_addr.to_string().starts_with("nft") {
                    self.handle_cw721(contract_addr, msg)
                } else {
                    self.base.handle_query(request)
                }
            }
            _ => self.base.handle_query(request),
        }
    }

    fn handle_cw721(&self, _contract_addr: &String, msg: &Binary) -> QuerierResult {
        match from_binary(&msg).unwrap() {
            Cw721QueryMsg::OwnerOf { token_id, .. } => {
                let owner = self
                    .nft_querier
                    .token_to_owner
                    .get(&token_id)
                    .map(|owner| owner.to_string())
                    .unwrap_or("".to_string());

                return QuerierResult::Ok(ContractResult::from(to_binary(&OwnerOfResponse {
                    owner: owner.clone(),
                    approvals: vec![],
                })));
            }
            _ => panic!("Unsupported CW721 query"),
        }
    }
}

impl MockWasmQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        MockWasmQuerier {
            base,
            nft_querier: NftQuerier::new(HashMap::new()),
        }
    }

    pub fn with_token_owners(&mut self, token_to_owner: HashMap<String, String>) {
        self.nft_querier = NftQuerier::new(token_to_owner);
    }
}
