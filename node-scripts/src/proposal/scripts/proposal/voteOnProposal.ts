import { getLogger } from '../../../common/logger';
import executeContract from '../../../utils/executeContract';
import { getWallet } from '../../../utils/wallet';
import { getProposalExecuteMsg } from '../../bindings/proposal/messages';

const logger = getLogger('voteOnProposal');

const PROPOSAL_CONTRACT_ADDR = 'terra1p8ssjr8rdeyksxujhgp4wqeslsqzx95fcm98s0';
const TOKEN_ID = '107697496010052054035518846537990279739';
const OPTION_ID = 0;

async function voteOnProposal() {
  const wallet = getWallet();

  await executeContract({
    contractAddress: PROPOSAL_CONTRACT_ADDR,
    operations: [
      {
        message: getProposalExecuteMsg('vote', {
          token_id: TOKEN_ID,
          option_id: OPTION_ID,
        }),
      },
    ],
    wallet,
  });
}

voteOnProposal().then(() => {
  logger.info('Done!');
});
