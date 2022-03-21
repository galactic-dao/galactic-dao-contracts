import uploadContract from '../../utils/uploadContract';
import { getWallet } from '../../utils/wallet';

const fp =
  '/Users/frankjia/Desktop/Programming/galaxy-labs/galactic-dao-contracts/artifacts/galacticdao_proposal_nft_voting.wasm';
const wallet = getWallet();

uploadContract(fp, wallet).then((codeId) => {
  console.log(codeId);
});
