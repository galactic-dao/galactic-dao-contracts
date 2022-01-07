#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{
        coins, from_binary, to_binary, Binary, CosmosMsg, DepsMut, Env, OwnedDeps, Response,
        SubMsg, Timestamp, Uint128, WasmMsg,
    };
    use cw20::Cw20ReceiveMsg;
    use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};
    use galacticdao_nft_staking_protocol::staking::{
        StakedNft, StakedNftState, StakingConfig, StakingExecuteMsg, StakingInstantiateMsg,
        StakingQueryMsg, TokenDistribution,
    };

    // Addresses
    const NFT: &str = "nft";
    const TOKEN_1: &str = "token_1";
    const TOKEN_2: &str = "token_2";
    const OWNER: &str = "owner";
    const TRUSTED_SENDER: &str = "trusted_sender";
    const STAKER_1: &str = "staker_1";
    const STAKER_2: &str = "staker_2";
    const PUBLIC: &str = "public";

    const NFT_TOKEN_ID_1: &str = "nft_1";
    const NFT_TOKEN_ID_2: &str = "nft_2";
    const NFT_TOKEN_ID_3: &str = "nft_3";

    fn instantiate_for_test(deps: DepsMut, init_cfg: &StakingConfig) {
        let init_msg = StakingInstantiateMsg {
            nft_contract: init_cfg.nft_contract.clone(),
            whitelisted_tokens: init_cfg.whitelisted_tokens.clone(),
            trusted_token_sender: init_cfg.trusted_token_sender.clone(),
            reward_withdrawal_timeout: init_cfg.reward_withdrawal_timeout,
        };

        instantiate(deps, mock_env(), mock_info(OWNER, &[]), init_msg).unwrap();
    }

    fn stake_nft_for_test(
        deps: DepsMut,
        env: Env,
        staker: &str,
        token_id: &str,
        nft: &str,
    ) -> Result<Response, ContractError> {
        execute(
            deps,
            env.clone(),
            mock_info(nft, &[]),
            StakingExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
                sender: staker.to_string(),
                token_id: token_id.to_string(),
                msg: Binary::default(),
            }),
        )
    }

    #[test]
    fn test_instantiate_and_change_cfg() {
        let mut deps = mock_dependencies(&[]);

        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![],
            trusted_token_sender: OWNER.to_string(),
            reward_withdrawal_timeout: 1000,
        };

        // Create with initial config
        instantiate_for_test(deps.as_mut(), &init_cfg);

        let query_msg = StakingQueryMsg::Config {};
        let cfg_res: StakingConfig =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(cfg_res, init_cfg);

        // Change config with non-owner - should fail
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(PUBLIC, &[]),
            StakingExecuteMsg::ChangeConfig {
                whitelisted_tokens: Some(vec![TOKEN_1.to_string()]),
                trusted_token_sender: None,
                reward_withdrawal_timeout: None,
            },
        )
        .expect_err("should not be able to change config as public");

        // Change config with fields
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER, &[]),
            StakingExecuteMsg::ChangeConfig {
                whitelisted_tokens: Some(vec![TOKEN_1.to_string()]),
                trusted_token_sender: None,
                reward_withdrawal_timeout: Some(2000),
            },
        )
        .unwrap();

        let query_msg = StakingQueryMsg::Config {};
        let cfg_res: StakingConfig =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(
            cfg_res,
            StakingConfig {
                owner: OWNER.to_string(),
                nft_contract: NFT.to_string(),
                whitelisted_tokens: vec![TOKEN_1.to_string()],
                trusted_token_sender: OWNER.to_string(),
                reward_withdrawal_timeout: 2000,
            }
        );
    }

    #[test]
    fn test_stake_nfts() {
        let mut deps = mock_dependencies(&[]);

        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![TOKEN_1.to_string()],
            trusted_token_sender: OWNER.to_string(),
            reward_withdrawal_timeout: 1000,
        };
        instantiate_for_test(deps.as_mut(), &init_cfg);

        // Cannot stake random nft
        stake_nft_for_test(deps.as_mut(), mock_env(), STAKER_1, NFT_TOKEN_ID_1, PUBLIC)
            .expect_err("Should not be able to stake random NFT");

        // Test query by address empty
        let query_msg = StakingQueryMsg::StakedByAddr {
            address: STAKER_1.to_string(),
        };
        let query_response: Vec<StakedNftState> =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response.len(), 0);

        // Test query num staked
        let query_msg = StakingQueryMsg::NumStaked {};
        let query_response: u64 =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response, 0);

        // Stake 2 nfts from different stakers
        let env = mock_env();
        let first_stake_time = env.block.time.seconds();
        stake_nft_for_test(deps.as_mut(), env.clone(), STAKER_1, NFT_TOKEN_ID_1, NFT).unwrap();
        stake_nft_for_test(deps.as_mut(), env.clone(), STAKER_2, NFT_TOKEN_ID_2, NFT).unwrap();

        // Stake another NFT at later time
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(first_stake_time + 10);
        let second_stake_time = env.block.time.seconds();
        stake_nft_for_test(deps.as_mut(), env.clone(), STAKER_1, NFT_TOKEN_ID_3, NFT).unwrap();

        // Test query num staked
        let query_msg = StakingQueryMsg::NumStaked {};
        let query_response: u64 =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response, 3);

        // Test query by token 1
        let query_msg = StakingQueryMsg::StakedByToken {
            token_id: NFT_TOKEN_ID_1.to_string(),
        };
        let query_response: StakedNftState =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(
            query_response.stake,
            StakedNft {
                token_id: NFT_TOKEN_ID_1.to_string(),
                time_deposited: first_stake_time,
                can_withdraw_rewards_time: first_stake_time + 1000,
                owner: STAKER_1.to_string(),
                last_claim_time: first_stake_time
            }
        );
        assert_eq!(query_response.unclaimed_rewards, vec![]);

        // Test query by address
        let query_msg = StakingQueryMsg::StakedByAddr {
            address: STAKER_1.to_string(),
        };
        let query_response: Vec<StakedNftState> =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(
            query_response[1].stake,
            StakedNft {
                token_id: NFT_TOKEN_ID_3.to_string(),
                time_deposited: second_stake_time,
                can_withdraw_rewards_time: second_stake_time + 1000,
                owner: STAKER_1.to_string(),
                last_claim_time: second_stake_time
            }
        );
        assert_eq!(query_response[1].unclaimed_rewards, vec![]);

        // Test get all with limit
        let query_msg = StakingQueryMsg::AllStaked {
            start_after_token: None,
            limit: Some(1),
        };
        let query_response: Vec<StakedNftState> =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response.len(), 1);
        assert_eq!(
            query_response[0].stake,
            StakedNft {
                token_id: NFT_TOKEN_ID_1.to_string(),
                time_deposited: first_stake_time,
                can_withdraw_rewards_time: first_stake_time + 1000,
                owner: STAKER_1.to_string(),
                last_claim_time: first_stake_time
            }
        );
        // Pagination
        let query_msg = StakingQueryMsg::AllStaked {
            start_after_token: Some(NFT_TOKEN_ID_1.to_string()),
            limit: Some(1),
        };
        let query_response: Vec<StakedNftState> =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response.len(), 1);
        assert_eq!(
            query_response[0].stake,
            StakedNft {
                token_id: NFT_TOKEN_ID_2.to_string(),
                time_deposited: first_stake_time,
                can_withdraw_rewards_time: first_stake_time + 1000,
                owner: STAKER_2.to_string(),
                last_claim_time: first_stake_time
            }
        );
    }

    #[test]
    fn test_stake_and_unstake() {
        let mut deps = mock_dependencies(&[]);

        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![TOKEN_1.to_string()],
            trusted_token_sender: OWNER.to_string(),
            reward_withdrawal_timeout: 1000,
        };
        instantiate_for_test(deps.as_mut(), &init_cfg);

        stake_nft_for_test(deps.as_mut(), mock_env(), STAKER_1, NFT_TOKEN_ID_1, NFT).unwrap();

        let query_msg = StakingQueryMsg::NumStaked {};
        let query_response: u64 =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response, 1);

        // Unstake not allowed from other wallets
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(PUBLIC, &[]),
            StakingExecuteMsg::WithdrawNft {
                token_id: NFT_TOKEN_ID_1.to_string(),
            },
        )
        .expect_err("Cannot call unstake on behalf of someone");

        // Unstake
        let unstake_response = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(STAKER_1, &[]),
            StakingExecuteMsg::WithdrawNft {
                token_id: NFT_TOKEN_ID_1.to_string(),
            },
        )
        .unwrap();
        assert_eq!(unstake_response.messages.len(), 1);
        assert_eq!(
            unstake_response.messages[0],
            SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: NFT.to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: STAKER_1.to_string(),
                    token_id: NFT_TOKEN_ID_1.to_string(),
                })
                .unwrap(),
                funds: vec![],
            }))
        );

        let query_msg = StakingQueryMsg::NumStaked {};
        let query_response: u64 =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response, 0);

        let query_msg = StakingQueryMsg::StakedByAddr {
            address: STAKER_1.to_string(),
        };
        let query_response: Vec<StakedNftState> =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response.len(), 0);

        let query_msg = StakingQueryMsg::AllStaked {
            start_after_token: None,
            limit: None,
        };
        let query_response: Vec<StakedNftState> =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_response.len(), 0);
    }

    #[test]
    fn test_distribute_tokens() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(OWNER, &[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            // TODO: test can't send unwhitelisted and non-trusted
            whitelisted_tokens: vec![TOKEN_1.to_string()],
            trusted_token_sender: OWNER.to_string(),
            reward_withdrawal_timeout: 1000,
        };
        let init_msg = StakingInstantiateMsg {
            nft_contract: init_cfg.nft_contract.clone(),
            whitelisted_tokens: init_cfg.whitelisted_tokens.clone(),
            trusted_token_sender: init_cfg.trusted_token_sender.clone(),
            reward_withdrawal_timeout: init_cfg.reward_withdrawal_timeout,
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let env = mock_env();
        let stake_time = env.block.time.seconds();
        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(NFT, &[]),
            StakingExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
                sender: STAKER_1.to_string(),
                token_id: NFT_TOKEN_ID_1.to_string(),
                msg: Binary::default(),
            }),
        )
        .unwrap();

        // Distribute
        let mut first_distribution_env = mock_env();
        let first_distribution_amt = Uint128::from(1000u32);
        let first_distribution_time = stake_time + 10;
        first_distribution_env.block.time = Timestamp::from_seconds(first_distribution_time);

        execute(
            deps.as_mut(),
            first_distribution_env.clone(),
            mock_info(TOKEN_1, &[]),
            StakingExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: OWNER.to_string(),
                amount: first_distribution_amt.clone(),
                msg: Binary::default(),
            }),
        )
        .unwrap();

        // Query to make sure we have the correct item saved
        let distributions_query: Vec<TokenDistribution> = from_binary(
            &query(
                deps.as_ref(),
                first_distribution_env.clone(),
                StakingQueryMsg::Distributions {
                    after_time: Some(first_distribution_time - 1),
                    limit: None,
                    token_addr: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(distributions_query.len(), 1 as usize);

        // Next query will be at first_distribution_time, and should return nothing
        let distributions_query: Vec<TokenDistribution> = from_binary(
            &query(
                deps.as_ref(),
                first_distribution_env.clone(),
                StakingQueryMsg::Distributions {
                    after_time: Some(first_distribution_time),
                    limit: None,
                    token_addr: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(distributions_query.len(), 0 as usize);

        // Now distribute again
        let mut second_distribution_env = mock_env();
        let second_distribution_amt = Uint128::from(2000u32);
        let second_distribution_time = first_distribution_time + 10;
        second_distribution_env.block.time = Timestamp::from_seconds(second_distribution_time);

        execute(
            deps.as_mut(),
            second_distribution_env.clone(),
            mock_info(TOKEN_1, &[]),
            StakingExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: OWNER.to_string(),
                amount: second_distribution_amt.clone(),
                msg: Binary::default(),
            }),
        )
        .unwrap();

        // Now query at first_distribution_time, and should return second distribution
        let distributions_query: Vec<TokenDistribution> = from_binary(
            &query(
                deps.as_ref(),
                first_distribution_env.clone(),
                StakingQueryMsg::Distributions {
                    after_time: Some(first_distribution_time),
                    limit: None,
                    token_addr: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            distributions_query[0].per_token_balance.amount,
            Uint128::from(2000u32)
        );

        // Test filtering by token
        // Test filtering by time
    }

    #[test]
    fn test_stake_distribute_and_withdraw() {
        // Test can't withdraw before timeout
    }

    #[test]
    fn test_owner_withdraw() {}
}
