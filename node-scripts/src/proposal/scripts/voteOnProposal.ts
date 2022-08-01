import { getLogger } from '../../common/logger';
import executeContract from '../../utils/executeContract';
import { getWallet } from '../../utils/wallet';
import { ProposalExecuteMsg } from '../../bindings/ProposalContract';

const logger = getLogger('voteOnProposal');

const PROPOSAL_CONTRACT_ADDR = 'terra1p8ssjr8rdeyksxujhgp4wqeslsqzx95fcm98s0';
const TOKEN_ID = '107697496010052054035518846537990279739';
const OPTION_ID = 0;

async function voteOnProposal() {
  const wallet = getWallet();
  const executeMsg: ProposalExecuteMsg = {
    vote: {
      token_id: TOKEN_ID,
      option_id: OPTION_ID,
    },
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

voteOnProposal().then(() => {
  logger.info('Done!');
});
