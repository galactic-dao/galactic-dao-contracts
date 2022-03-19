import { getLogger } from '../../../common/logger';
import executeContract from '../../../utils/executeContract';
import { getWallet } from '../../../utils/wallet';
import { getProposalExecuteMsg } from '../../bindings/proposal/messages';

const logger = getLogger('revokeProposal');
const PROPOSAL_CONTRACT_ADDR = 'terra1p8ssjr8rdeyksxujhgp4wqeslsqzx95fcm98s0';

async function revokeProposal() {
  const wallet = getWallet();

  await executeContract({
    contractAddress: PROPOSAL_CONTRACT_ADDR,
    operations: [
      {
        message: getProposalExecuteMsg('revoke', {}),
      },
    ],
    wallet,
  });
}

revokeProposal().then(() => {
  logger.info('Done!');
});
