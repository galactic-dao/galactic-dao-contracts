import uploadContract from '../utils/uploadContract';
import { defaultWallet } from '../utils/wallet';

const main = async () => {
  const fp = '';

  const codeId = await uploadContract(fp, defaultWallet);
  console.log(codeId);
};

main();
