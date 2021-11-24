import { ProposalExecuteVoteMessage, ProposalQueryVotesMessage } from "./proposalTypes";

/*
Execute
 */
export function getProposalExecuteVoteMsg(params: ProposalExecuteVoteMessage) {
  return {
    vote: params,
  };
}

export function getProposalRevokeMsg() {
  return {
    revoke: {},
  };
}

/*
Query
 */
export function getProposalQueryStatusMsg() {
  return {
    status: {},
  };
}

export function getProposalQueryVotesMsg(params: ProposalQueryVotesMessage) {
  return {
    votes: params,
  };
}
