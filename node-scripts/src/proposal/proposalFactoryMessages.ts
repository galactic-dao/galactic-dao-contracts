import {
  ProposalFactoryExecuteCreateMessage,
  ProposalFactoryExecuteModifyConfigMessage,
  ProposalFactoryExecuteWithdrawMessage
} from "./proposalFactoryTypes";

/*
Execute
 */
export function getProposalFactoryExecuteCreateMsg(
  params: ProposalFactoryExecuteCreateMessage
) {
  return {
    create_proposal: params,
  };
}

export function getProposalFactoryModifyConfigMsg(
  params: ProposalFactoryExecuteModifyConfigMessage
) {
  return {
    modify_config: params,
  };
}

export function getProposalFactoryWithdrawMsg(
  params: ProposalFactoryExecuteWithdrawMessage
) {
  return {
    withdraw_funds: params,
  };
}

/*
Query
 */
export function getProposalFactoryQueryStatusMsg() {
  return {
    status: {},
  };
}
