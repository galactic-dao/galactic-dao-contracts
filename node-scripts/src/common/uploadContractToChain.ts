import uploadContract from '../utils/uploadContract';
import { defaultWallet } from '../utils/wallet';

const main = async () => {
  const fp =
    '/Users/frankjia/Desktop/Programming/galaxy-labs/galactic-dao-contracts/artifacts/galacticdao_proposal_nft_voting_factory.wasm';

  const codeId = await uploadContract(fp, defaultWallet);
  console.log(codeId);
};

main();
