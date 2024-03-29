import { getLogger } from '../../common/logger';
import { getWallet } from '../../utils/wallet';
import { nftProposalCodeIds } from '../constants';
import { environment } from '../../utils/environment';
import instantiateContract from '../../utils/instantiateContract';
import {
  ProposalInstantiateMsg,
  ProposalOption,
} from '../../bindings/ProposalContract';

const PROPOSAL_OPTIONS: ProposalOption[] = [
  {
    id: 0,
    name: 'Abstain',
  },
  {
    id: 1,
    name: 'Yes',
  },
  {
    id: 2,
    name: 'No',
  },
];

const CLOSE_TIME = Math.round(Date.now() / 1000 + 60 * 60 * 24 * 3); // 3 Days

const logger = getLogger('instantiateProposalContract');

async function instantiateProposalContract() {
  const wallet = getWallet();
  const codeId = nftProposalCodeIds[environment.chainType];

  console.log('Proposer', wallet.key.accAddress);
  console.log('Start date', new Date().toISOString());
  console.log('Close date', new Date(CLOSE_TIME * 1000).toISOString());

  const instantiateMsg: ProposalInstantiateMsg = {
    config: {
      // GP testnet contracts
      nft_contract:
        'terra1c0dnkxd0urua9rfn7yn60ktedgqrl9zlnlvuwxpq7ykjtygyjgyq35kr2p',
      // GP mainnet contracts
      // nft_contract: 'terra16ds898j530kn4nnlc7xlj6hcxzqpcxxk4mj8gkcl3vswksu6s3zszs8kp2',
      title: 'TEST PROPOSAL',
      proposal_uri:
        'ipfs://bafkreihm3zutqqvjrh2g5n4bxri3l6ptso65s5hvx64ytyg5lrnb2fkbxq',
      options: PROPOSAL_OPTIONS,
      quorum_fraction: '0.05',
      close_time: CLOSE_TIME,
    },
    proposer: wallet.key.accAddress,
  };

  const contract = await instantiateContract({
    contractCodeId: codeId,
    initMessage: instantiateMsg,
    label: 'Galactic Punks Proposal',
    wallet,
  });

  logger.info('Contract instantiated', contract);
}

instantiateProposalContract().then(() => {
  logger.info('Done!');
});
