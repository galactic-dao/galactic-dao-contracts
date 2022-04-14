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
    name: 'Yes',
  },
  {
    id: 2,
    name: 'No',
  },
];

const CLOSE_TIME = Math.round(Date.parse('22 Apr 2022 00:00:00 GMT') / 1000);

const logger = getLogger('instantiateProposalContract');

async function instantiateProposalContract() {
  const wallet = getWallet();
  const codeId = nftProposalCodeIds[environment.chainType];

  const instantiateMsg: ProposalInstantiateParams = {
    config: {
      // GP testnet contracts
      nft_contract: 'terra1ka7r34ysylvnujtat9gjkydteny7f6g7xrk5ea',
      nft_staking_contract: 'terra12edeeudxj0f5uyfgye098crhzy080hvjmw9vt7',
      title: 'Refract 1000 LUNA & Stake yLUNA into PRISM Farm',
      proposal_uri:
        'ipfs://bafkreifdapicm7lg5us7twbl47kbqwsvu6zs52vzfrmoytazmx75u5ghmu',
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
