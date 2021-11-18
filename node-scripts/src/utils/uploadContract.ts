import { isTxError, MsgStoreCode, Wallet } from "@terra-money/terra.js";
import * as fs from "fs";

/*
Uploads a given contract and returns the code_id of the contract
 */
const uploadContract = async (
  filepath: string,
  wallet: Wallet
): Promise<string> => {
  const storeCode = new MsgStoreCode(
    wallet.key.accAddress,
    fs.readFileSync(filepath).toString('base64')
  );
  const storeCodeTx = await wallet.createAndSignTx({
    msgs: [storeCode],
  });
  const storeCodeTxResult = await wallet.lcd.tx.broadcast(storeCodeTx);

  console.log('Code uploaded', storeCodeTxResult);

  if (isTxError(storeCodeTxResult)) {
    throw new Error(
      `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
    );
  }

  const {
    store_code: { code_id },
  } = storeCodeTxResult.logs[0].eventsByType;

  return code_id[0];
};

export default uploadContract;
