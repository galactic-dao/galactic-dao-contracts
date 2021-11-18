import { Coins } from '@terra-money/terra.js/dist/core/Coins';
import { isTxError, MsgExecuteContract, Wallet } from '@terra-money/terra.js';
import { BlockTxBroadcastResult } from '@terra-money/terra.js/dist/client/lcd/api/TxAPI';

export type ExecuteContractOperation = {
  message: Record<string, any>;
  coins?: Coins.Input;
};
export type ExecuteContractVariables = {
  contractAddress: string;
  operations: Array<ExecuteContractOperation>;
  wallet: Wallet;
};

/*
Execute the contract with given messages. Execution happens in ALL or NOTHING
fashion - so if one message fails, the entire txn is rolled back.
 */
const executeContract = async ({
  contractAddress,
  operations,
  wallet,
}: ExecuteContractVariables): Promise<BlockTxBroadcastResult> => {
  console.log(
    'Execute contract with params',
    JSON.stringify({
      contractAddress: contractAddress,
      operations: operations,
    })
  );

  const executeMessages: MsgExecuteContract[] = operations.map((op) => {
    return new MsgExecuteContract(
      wallet.key.accAddress, // sender
      contractAddress,
      op.message,
      op.coins
    );
  });

  const executeTx = await wallet.createAndSignTx({
    msgs: executeMessages,
  });

  const executeTxResult = await wallet.lcd.tx.broadcast(executeTx);

  if (isTxError(executeTxResult)) {
    throw new Error(
      `Execute transaction failed. code: ${executeTxResult.code}, codespace: ${executeTxResult.codespace}, raw_log: ${executeTxResult.raw_log}`
    );
  }

  console.log('Transaction executed with result', executeTxResult.logs[0]);

  return executeTxResult;
};

export default executeContract;
