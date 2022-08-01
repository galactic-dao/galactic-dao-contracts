import { getLogger } from '../../common/logger';
import executeContract from '../../utils/executeContract';
import { getWallet } from '../../utils/wallet';
import { ProposalExecuteMsg } from '../../bindings/ProposalContract';

const logger = getLogger('revokeProposal');
const PROPOSAL_CONTRACT_ADDR = 'terra1p8ssjr8rdeyksxujhgp4wqeslsqzx95fcm98s0';

async function revokeProposal() {
  const wallet = getWallet();

  const executeMsg: ProposalExecuteMsg = {
    revoke: {},
  };
  await executeContract({
    contractAddress: PROPOSAL_CONTRACT_ADDR,
    operations: [
      {
        message: executeMsg,
      },
    ],
    wallet,
  });
}

revokeProposal().then(() => {
  logger.info('Done!');
});
