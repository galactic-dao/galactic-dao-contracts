#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, from_binary, Binary, OwnedDeps, Timestamp, Uint128};
    use cw20::Cw20ReceiveMsg;
    use cw721::Cw721ReceiveMsg;
    use galacticdao_nft_staking_protocol::staking::{
        StakedNft, StakedNftState, StakingConfig, StakingExecuteMsg, StakingInstantiateMsg,
        StakingQueryMsg, TokenDistribution,
    };

    // Addresses
    const NFT: &str = "nft";
    const TOKEN_1: &str = "token_1";
    const OWNER: &str = "owner";
    const TRUSTED_SENDER: &str = "trusted_sender";
    const STAKER: &str = "staker";
    const PUBLIC: &str = "public";

    const NFT_TOKEN_ID_1: &str = "1";

    // TODO: Util fns: https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/contract.rs#L561

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(OWNER, &[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
            whitelisted_tokens: vec![],
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

        let query_msg = StakingQueryMsg::Config {};
        let cfg_res: StakingConfig =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(cfg_res, init_cfg);
    }

    #[test]
    fn test_stake_nft() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(OWNER, &[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
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
                sender: STAKER.to_string(),
                token_id: NFT_TOKEN_ID_1.to_string(),
                msg: Binary::default(),
            }),
        )
        .unwrap();

        let query_stake_msg = StakingQueryMsg::StakedByToken {
            token_id: NFT_TOKEN_ID_1.to_string(),
        };
        let stake_res: StakedNftState =
            from_binary(&query(deps.as_ref(), mock_env(), query_stake_msg).unwrap()).unwrap();
        assert_eq!(
            stake_res.stake,
            StakedNft {
                token_id: NFT_TOKEN_ID_1.to_string(),
                time_deposited: stake_time,
                can_withdraw_rewards_time: stake_time + 1000,
                owner: STAKER.to_string(),
                last_claim_time: stake_time
            }
        );
        assert_eq!(stake_res.unclaimed_rewards, vec![])
    }

    #[test]
    fn test_distribute_tokens() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(OWNER, &[]);
        let init_cfg = StakingConfig {
            owner: OWNER.to_string(),
            nft_contract: NFT.to_string(),
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
                sender: STAKER.to_string(),
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
    }
}
