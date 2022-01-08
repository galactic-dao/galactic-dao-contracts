import { BlockTxBroadcastResult, TxLog } from '@terra-money/terra.js';

/**
 * Parses the txlog from calling create_proposal on the factory to retrieve
 * the created proposal contract address
 */
export function getCreatedProposalAddress(
  broadcastResult: BlockTxBroadcastResult
): string {
  return (broadcastResult.logs[0] as TxLog).eventsByType[
    'instantiate_contract'
  ]['contract_address'][0];
}
