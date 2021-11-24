import { ProposalFactoryExecuteCreateMessage, ProposalFactoryExecuteModifyConfigMessage } from "./proposalFactoryTypes";

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

/*
Query
 */
export function getProposalFactoryQueryStatusMsg() {
  return {
    status: {},
  };
}
