import {
  ProposalInstantiateParams,
  ProposalOption,
} from '../../bindings/proposal/models';
import { getLogger } from '../../../common/logger';
import { getWallet } from '../../../utils/wallet';
import { nftProposalCodeIds } from '../../constants';
import { environment } from '../../../utils/environment';
import instantiateContract from '../../../utils/instantiateContract';

const PROPOSAL_OPTIONS: ProposalOption[] = [
  {
    id: 0,
    name: 'Abstain',
  },
  {
    id: 1,
    name: 'Approve',
  },
  {
    id: 2,
    name: 'Reject',
  },
];

const CLOSE_TIME = Math.round(Date.parse('01 Apr 2022 00:00:00 GMT') / 1000);

const logger = getLogger('instantiateProposalContract');

async function instantiateProposalContract() {
  const wallet = getWallet();
  const codeId = nftProposalCodeIds[environment.chainType];

  const instantiateMsg: ProposalInstantiateParams = {
    config: {
      // GP testnet contract
      nft_contract: 'terra1pk646xtdgwym74k46cajttdu6uvypa5jw5wa3j',
      title: 'Test Title',
      proposal_uri: '',
      options: PROPOSAL_OPTIONS,
      quorum_fraction: '0.2',
      close_time: CLOSE_TIME,
    },
    proposer: wallet.key.accAddress,
  };

  const contract = await instantiateContract({
    contractCodeId: codeId,
    initMessage: instantiateMsg,
    wallet,
  });

  logger.info('Contract instantiated', contract);
}

instantiateProposalContract().then(() => {
  logger.info('Done!');
});
