import uploadContract from '../../utils/uploadContract';
import { getWallet } from '../../utils/wallet';
import path from 'path';

const relPathsForContracts = {
  cw721: '../../../../external-artifacts/cw721_metadata_onchain.wasm',
  proposal: '../../../../artifacts/galacticdao_proposal_nft_voting.wasm',
};

const fp = path.join(__dirname, relPathsForContracts.cw721);
const wallet = getWallet();

uploadContract(fp, wallet).then((codeId) => {
  console.log(codeId);
});
