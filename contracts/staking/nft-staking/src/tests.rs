#[cfg(test)]
mod tests {
    use std::ops::Div;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{
        from_binary, to_binary, Binary, CosmosMsg, DepsMut, Env, Response, SubMsg, Timestamp,
        Uint128, WasmMsg,
    };
    use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
    use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};

    use galacticdao_nft_staking_protocol::staking::{
        StakedNft, StakedNftState, StakingConfig, StakingExecuteMsg, StakingInstantiateMsg,
        StakingQueryMsg, TokenBalance,
    };

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;

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

    fn distribute_token_for_test(
        deps: DepsMut,
        env: Env,
        distributor: &str,
        token: &str,
        amount: &Uint128,
    ) -> Result<Response, ContractError> {
        execute(
            deps,
            env.clone(),
            mock_info(token, &[]),
            StakingExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: distributor.to_string(),
                amount: amount.clone(),
                msg: Binary::default(),
            }),
        )
    }

    fn assert_token_balance(balances: &Vec<TokenBalance>, expected: &TokenBalance) {
        assert_eq!(
            balances
                .iter()
                .find(|x| x.token == expected.token.to_string())
                .unwrap(),
            expected
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
    fn test_stake_and_query() {
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
            start_after_token: None,
            limit: None,
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
                beginning_reward_snapshot: vec![],
                last_reward_snapshot: vec![]
            }
        );
        assert_eq!(query_response.unclaimed_rewards, vec![]);

        // Test query by address
        let query_msg = StakingQueryMsg::StakedByAddr {
            address: STAKER_1.to_string(),
            start_after_token: None,
            limit: None,
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
                beginning_reward_snapshot: vec![],
                last_reward_snapshot: vec![]
            }
        );
        assert_eq!(query_response[1].unclaimed_rewards, vec![]);

        // Test query by address with limit of 1 and with start_after
        let query_response: Vec<StakedNftState> = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                StakingQueryMsg::StakedByAddr {
                    address: STAKER_1.to_string(),
                    start_after_token: Some(NFT_TOKEN_ID_1.to_string()),
                    limit: None,
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(query_response.len(), 1);
        let query_response: Vec<StakedNftState> = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                StakingQueryMsg::StakedByAddr {
                    address: STAKER_1.to_string(),
                    start_after_token: None,
                    limit: Some(1),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(query_response.len(), 1);

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
                beginning_reward_snapshot: vec![],
                last_reward_snapshot: vec![]
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
                beginning_reward_snapshot: vec![],
                last_reward_snapshot: vec![]
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
            start_after_token: None,
            limit: None,
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
    fn test_distribute_and_query() {
        let mut deps = mock_dependencies(&[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![],
            trusted_token_sender: TRUSTED_SENDER.to_string(),
            reward_withdrawal_timeout: 1000,
        };

        instantiate_for_test(deps.as_mut(), &init_cfg);

        // Test that distributions cannot be sent from non-whitelisted tokens
        distribute_token_for_test(
            deps.as_mut(),
            mock_env(),
            TRUSTED_SENDER,
            PUBLIC,
            &Uint128::from(1000u64),
        )
        .expect_err("should not be able to distribute non-whitelisted token");

        // Now whitelist
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER, &[]),
            StakingExecuteMsg::ChangeConfig {
                whitelisted_tokens: Some(vec![TOKEN_1.to_string(), TOKEN_2.to_string()]),
                trusted_token_sender: None,
                reward_withdrawal_timeout: None,
            },
        )
        .unwrap();

        // Test that distributions cannot be sent from addresses other than trusted sender
        distribute_token_for_test(
            deps.as_mut(),
            mock_env(),
            OWNER,
            TOKEN_1,
            &Uint128::from(1000u64),
        )
        .expect_err("should not be able to distribute from non-trusted sender");

        // Stake an NFT
        let stake_env = mock_env();
        let first_stake_time = stake_env.block.time.seconds();
        stake_nft_for_test(
            deps.as_mut(),
            stake_env.clone(),
            STAKER_1,
            NFT_TOKEN_ID_1,
            NFT,
        )
        .unwrap();

        // Distribute in a valid case
        let mut first_distribution_env = mock_env();
        first_distribution_env.block.time = Timestamp::from_seconds(first_stake_time + 10);
        let first_distribution_amt = Uint128::from(1000u32);
        let first_distribution_time = first_distribution_env.block.time.seconds();
        distribute_token_for_test(
            deps.as_mut(),
            first_distribution_env.clone(),
            TRUSTED_SENDER,
            TOKEN_1,
            &first_distribution_amt,
        )
        .unwrap();

        // Query to make sure we have the correct distribution saved
        let total_rewards_query: Vec<TokenBalance> = from_binary(
            &query(
                deps.as_ref(),
                first_distribution_env.clone(),
                StakingQueryMsg::TotalRewards {},
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(total_rewards_query.len(), 1 as usize);
        assert_eq!(
            total_rewards_query[0],
            TokenBalance {
                amount: first_distribution_amt.clone(),
                token: TOKEN_1.to_string()
            }
        );

        // Stake another NFT
        let mut stake_env = mock_env();
        stake_env.block.time = Timestamp::from_seconds(first_distribution_time + 10);
        stake_nft_for_test(
            deps.as_mut(),
            stake_env.clone(),
            STAKER_2,
            NFT_TOKEN_ID_2,
            NFT,
        )
        .unwrap();

        // Check that second stake is not eligible for this reward
        let query_resp: StakedNftState = from_binary(
            &query(
                deps.as_ref(),
                first_distribution_env.clone(),
                StakingQueryMsg::StakedByToken {
                    token_id: NFT_TOKEN_ID_2.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            query_resp.unclaimed_rewards,
            vec![TokenBalance {
                amount: Uint128::zero(),
                token: TOKEN_1.to_string()
            }]
        );

        // Now distribute again
        let mut second_distribution_env = mock_env();
        second_distribution_env.block.time = second_distribution_env.block.time.plus_seconds(100);
        let second_distribution_amt = Uint128::from(665u32);
        let _second_distribution_time = second_distribution_env.block.time.seconds();
        distribute_token_for_test(
            deps.as_mut(),
            second_distribution_env.clone(),
            TRUSTED_SENDER,
            TOKEN_2,
            &second_distribution_amt,
        )
        .unwrap();

        // Now both staked should be eligible
        let query_resp: StakedNftState = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                StakingQueryMsg::StakedByToken {
                    token_id: NFT_TOKEN_ID_1.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_token_balance(
            &query_resp.unclaimed_rewards,
            &TokenBalance {
                amount: first_distribution_amt.clone(),
                token: TOKEN_1.to_string(),
            },
        );
        assert_token_balance(
            &query_resp.unclaimed_rewards,
            &TokenBalance {
                amount: second_distribution_amt.div(Uint128::from(2u8)),
                token: TOKEN_2.to_string(),
            },
        );

        let query_resp: StakedNftState = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                StakingQueryMsg::StakedByToken {
                    token_id: NFT_TOKEN_ID_2.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(query_resp.unclaimed_rewards.len(), 2 as usize);
        assert_token_balance(
            &query_resp.unclaimed_rewards,
            &TokenBalance {
                amount: Uint128::zero(),
                token: TOKEN_1.to_string(),
            },
        );
        assert_token_balance(
            &query_resp.unclaimed_rewards,
            &TokenBalance {
                amount: second_distribution_amt.div(Uint128::from(2u8)),
                token: TOKEN_2.to_string(),
            },
        );

        // check total rewards again
        let total_rewards_query: Vec<TokenBalance> = from_binary(
            &query(
                deps.as_ref(),
                first_distribution_env.clone(),
                StakingQueryMsg::TotalRewards {},
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(total_rewards_query.len(), 2 as usize);
        assert_eq!(
            total_rewards_query[1],
            TokenBalance {
                amount: second_distribution_amt.div(Uint128::from(2u8)),
                token: TOKEN_2.to_string()
            }
        );
    }

    #[test]
    fn test_stake_distribute_and_withdraw() {
        let mut deps = mock_dependencies(&[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![TOKEN_1.to_string()],
            trusted_token_sender: TRUSTED_SENDER.to_string(),
            reward_withdrawal_timeout: 1000,
        };

        // Instantiate, stake, distribute
        instantiate_for_test(deps.as_mut(), &init_cfg);
        stake_nft_for_test(deps.as_mut(), mock_env(), STAKER_1, TOKEN_1, NFT).unwrap();
        let mut distribution_env = mock_env();
        distribution_env.block.time = distribution_env.block.time.plus_seconds(10);
        let distribution_amt = Uint128::from(1000u32);
        distribute_token_for_test(
            deps.as_mut(),
            distribution_env.clone(),
            TRUSTED_SENDER,
            TOKEN_1,
            &distribution_amt,
        )
        .unwrap();

        // Test can't withdraw before timeout
        let mut lockup_period_env = mock_env();
        lockup_period_env.block.time = distribution_env.block.time.plus_seconds(10);
        execute(
            deps.as_mut(),
            lockup_period_env.clone(),
            mock_info(STAKER_1, &[]),
            StakingExecuteMsg::WithdrawRewards {
                token_id: TOKEN_1.to_string(),
            },
        )
        .expect_err("should not be able to withdraw before lockup timeout");

        let mut valid_withdraw_env = mock_env();
        valid_withdraw_env.block.time = valid_withdraw_env
            .block
            .time
            .plus_seconds(init_cfg.reward_withdrawal_timeout);

        // Test can't withdraw for someone else
        execute(
            deps.as_mut(),
            valid_withdraw_env.clone(),
            mock_info(PUBLIC, &[]),
            StakingExecuteMsg::WithdrawRewards {
                token_id: TOKEN_1.to_string(),
            },
        )
        .expect_err("should not be able to withdraw for someone else");

        // Test withdraw has correct msg
        let withdraw_response = execute(
            deps.as_mut(),
            valid_withdraw_env.clone(),
            mock_info(STAKER_1, &[]),
            StakingExecuteMsg::WithdrawRewards {
                token_id: TOKEN_1.to_string(),
            },
        )
        .unwrap();
        assert_eq!(withdraw_response.messages.len(), 1);
        assert_eq!(
            withdraw_response.messages[0],
            SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: TOKEN_1.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: STAKER_1.to_string(),
                    amount: distribution_amt.clone()
                })
                .unwrap(),
                funds: vec![],
            }))
        );

        // Test once withdrawn, future withdrawals don't have the distribution
        let withdraw_response = execute(
            deps.as_mut(),
            valid_withdraw_env.clone(),
            mock_info(STAKER_1, &[]),
            StakingExecuteMsg::WithdrawRewards {
                token_id: TOKEN_1.to_string(),
            },
        )
        .unwrap();
        assert_eq!(withdraw_response.messages.len(), 0);
    }

    #[test]
    fn test_owner_withdraw() {
        let mut deps = mock_dependencies(&[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![TOKEN_1.to_string()],
            trusted_token_sender: TRUSTED_SENDER.to_string(),
            reward_withdrawal_timeout: 1000,
        };

        instantiate_for_test(deps.as_mut(), &init_cfg);

        // Test public can't call owner-only withdraw
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(PUBLIC, &[]),
            StakingExecuteMsg::OwnerWithdrawTokens {
                balance: TokenBalance {
                    amount: Uint128::from(10u32),
                    token: TOKEN_1.to_string(),
                },
            },
        )
        .expect_err("only owner should be able to call this method");

        // Test valid owner withdraw
        let withdraw_resp = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER, &[]),
            StakingExecuteMsg::OwnerWithdrawTokens {
                balance: TokenBalance {
                    amount: Uint128::from(10u32),
                    token: TOKEN_1.to_string(),
                },
            },
        )
        .unwrap();
        assert_eq!(withdraw_resp.messages.len(), 1);
        assert_eq!(
            withdraw_resp.messages[0],
            SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: TOKEN_1.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: OWNER.to_string(),
                    amount: Uint128::from(10u32)
                })
                .unwrap(),
                funds: vec![],
            }))
        );
    }
}
