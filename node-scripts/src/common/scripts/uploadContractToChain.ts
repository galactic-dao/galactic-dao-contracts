import uploadContract from '../../utils/uploadContract';
import { getWallet } from '../../utils/wallet';

const fp =
  '/Users/frankjia/Desktop/Programming/galaxy-labs/galactic-dao-contracts/artifacts/galacticdao_nft_staking.wasm';
const wallet = getWallet();

uploadContract(fp, wallet).then((codeId) => {
  console.log(codeId);
});
