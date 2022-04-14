#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{Decimal, DepsMut, Timestamp};

    use galacticdao_nft_voting_protocol::proposal::{
        ProposalConfig, ProposalExecuteMsg, ProposalInstantiateMsg, ProposalOption,
        ProposalOptionStatus, ProposalState, ProposalStatusResponse,
    };
    use galacticdao_nft_voting_protocol::testing::mock_dependencies;

    use crate::contract::{
        execute, execute_revoke, execute_vote, instantiate, query_status, query_votes,
    };

    const NFT_CONTRACT: &str = "nft_contract";
    const NFT_STAKING_CONTRACT: &str = "nft_staking_contract";
    const PROPOSAL_URI: &str = "proposal_uri";
    const TITLE: &str = "title";
    const PROPOSAL_CREATOR: &str = "creator";
    const PROPOSER: &str = "proposer";
    const CLOSE_TIME: u64 = 100;

    const VOTER_WITH_NFT: &str = "voter_with_nft";
    const VOTER_NFT_TOKEN_ID: &str = "nft_token_id";

    const PUBLIC: &str = "public";

    fn instantiate_default_proposal(deps: DepsMut) {
        let quorum: Decimal = Decimal::percent(10);
        let options: Vec<ProposalOption> = vec![
            ProposalOption {
                id: 0,
                name: "0".to_string(),
            },
            ProposalOption {
                id: 1,
                name: "1".to_string(),
            },
        ];

        let instantiate_msg = ProposalInstantiateMsg {
            config: ProposalConfig {
                nft_contract: NFT_CONTRACT.to_string(),
                nft_staking_contract: NFT_STAKING_CONTRACT.to_string(),
                title: TITLE.to_string(),
                proposal_uri: PROPOSAL_URI.to_string(),
                options: options.clone(),
                close_time: CLOSE_TIME,
                quorum_fraction: quorum,
            },
            proposer: PROPOSER.to_string(),
        };
        let info = mock_info(PROPOSAL_CREATOR, &[]);
        instantiate(deps, mock_env(), info, instantiate_msg).unwrap();
    }

    #[test]
    fn instantiate_and_query_status() {
        let mut deps = mock_dependencies(&[]);
        let expected_quorum: Decimal = Decimal::percent(10);
        let expected_options: Vec<ProposalOption> = vec![
            ProposalOption {
                id: 0,
                name: "0".to_string(),
            },
            ProposalOption {
                id: 1,
                name: "1".to_string(),
            },
        ];

        // Instantiate contract
        instantiate_default_proposal(deps.as_mut());

        // Ensure expected initial state
        let expected = ProposalStatusResponse {
            state: ProposalState {
                creator: PROPOSAL_CREATOR.to_string(),
                is_revoked: false,
                proposer: PROPOSER.to_string(),
            },
            config: ProposalConfig {
                nft_contract: NFT_CONTRACT.to_string(),
                nft_staking_contract: NFT_STAKING_CONTRACT.to_string(),
                title: TITLE.to_string(),
                proposal_uri: PROPOSAL_URI.to_string(),
                options: expected_options.clone(),
                close_time: CLOSE_TIME,
                quorum_fraction: expected_quorum,
            },
            tally: expected_options
                .clone()
                .iter()
                .map(|option| ProposalOptionStatus {
                    id: option.id,
                    votes: 0,
                })
                .collect(),
        };
        assert_eq!(query_status(deps.as_ref()).unwrap(), expected);
    }

    #[test]
    fn vote_with_token_owner() {
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_token_owners(HashMap::from([(
            VOTER_NFT_TOKEN_ID.to_string(),
            VOTER_WITH_NFT.to_string(),
        )]));

        instantiate_default_proposal(deps.as_mut());

        // Valid vote
        let mut first_vote_env = mock_env();
        first_vote_env.block.time = Timestamp::from_seconds(CLOSE_TIME - 10);
        execute(
            deps.as_mut(),
            first_vote_env.clone(),
            mock_info(VOTER_WITH_NFT, &[]),
            ProposalExecuteMsg::Vote {
                option_id: Some(0),
                token_id: VOTER_NFT_TOKEN_ID.to_string(),
            },
        )
        .unwrap();

        // Check tally
        let status_resp = query_status(deps.as_ref()).unwrap();
        assert_eq!(
            status_resp
                .tally
                .iter()
                .find(|tally| tally.id == 0)
                .unwrap()
                .votes,
            1
        );

        // Check vote by token ID
        let vote_resp = query_votes(deps.as_ref(), vec![VOTER_NFT_TOKEN_ID.to_string()]).unwrap();
        assert_eq!(vote_resp.votes[0], Some(0));

        // Now retract vote
        execute(
            deps.as_mut(),
            first_vote_env.clone(),
            mock_info(VOTER_WITH_NFT, &[]),
            ProposalExecuteMsg::Vote {
                option_id: None,
                token_id: VOTER_NFT_TOKEN_ID.to_string(),
            },
        )
        .unwrap();

        let status_resp = query_status(deps.as_ref()).unwrap();
        assert_eq!(
            status_resp
                .tally
                .iter()
                .find(|tally| tally.id == 0)
                .unwrap()
                .votes,
            0
        );
        let vote_resp = query_votes(deps.as_ref(), vec![VOTER_NFT_TOKEN_ID.to_string()]).unwrap();
        assert_eq!(vote_resp.votes[0], None);

        // Check can't vote after close time
        let mut second_vote_env = mock_env();
        second_vote_env.block.time = Timestamp::from_seconds(CLOSE_TIME + 10);
        execute(
            deps.as_mut(),
            second_vote_env.clone(),
            mock_info(VOTER_WITH_NFT, &[]),
            ProposalExecuteMsg::Vote {
                option_id: Some(0),
                token_id: VOTER_NFT_TOKEN_ID.to_string(),
            },
        )
        .expect_err("should not be able to vote after close time");
    }

    #[test]
    fn proper_auth_check() {
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_token_owners(HashMap::from([(
            VOTER_NFT_TOKEN_ID.to_string(),
            VOTER_WITH_NFT.to_string(),
        )]));
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(CLOSE_TIME - 10);

        instantiate_default_proposal(deps.as_mut());

        execute_vote(
            deps.as_mut(),
            env.clone(),
            mock_info(PUBLIC, &[]),
            None,
            &VOTER_NFT_TOKEN_ID.to_string(),
        )
        .expect_err("should not be able to vote without token ownership");

        execute_revoke(deps.as_mut(), env.clone(), mock_info(PUBLIC, &[]))
            .expect_err("should not be able to revoke as non-proposer");

        // Now revoke and check that voting is blocked
        execute_revoke(deps.as_mut(), env.clone(), mock_info(PROPOSER, &[])).unwrap();

        execute_vote(
            deps.as_mut(),
            env.clone(),
            mock_info(VOTER_WITH_NFT, &[]),
            Some(0),
            &VOTER_NFT_TOKEN_ID.to_string(),
        )
        .expect_err("should not be able to vote after proposal is revoked");

        // Check can't revoke after close time
        env.block.time = Timestamp::from_seconds(CLOSE_TIME + 10);
        execute_revoke(deps.as_mut(), env.clone(), mock_info(PROPOSER, &[]))
            .expect_err("should not be able to revoke after close time");
    }
}
