import {
  isTxError,
  MsgInstantiateContract,
  Wallet,
} from '@terra-money/terra.js';
import { Coins } from '@terra-money/terra.js/dist/core/Coins';
import { getLogger } from '../common/logger';

export type InstantiateContractVariables = {
  label: string;
  contractCodeId: number;
  initMessage: Record<string, any>;
  wallet: Wallet;
  initCoins?: Coins.Input;
};

const logger = getLogger('instantiateContract', 'debug');

/*
Instantiates a contract with a given code ID, init message, and init coins
 */
const instantiateContract = async ({
  contractCodeId,
  initMessage,
  wallet,
  initCoins,
  label,
}: InstantiateContractVariables): Promise<string> => {
  const instantiate = new MsgInstantiateContract(
    wallet.key.accAddress,
    wallet.key.accAddress,
    contractCodeId, // code ID in number form
    initMessage,
    initCoins,
    label
  );

  const instantiateTx = await wallet.createAndSignTx({
    msgs: [instantiate],
    gasAdjustment: 1.2,
  });
  const instantiateTxResult = await wallet.lcd.tx.broadcast(instantiateTx);

  logger.debug('Instantiated contract', instantiateTxResult);

  if (isTxError(instantiateTxResult)) {
    throw new Error(
      `instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}, raw_log: ${instantiateTxResult.raw_log}`
    );
  }

  const {
    instantiate: { _contract_address },
  } = instantiateTxResult.logs[0].eventsByType;

  return _contract_address[0];
};

export default instantiateContract;
