// @flow

import {sendAndConfirmTransaction as realSendAndConfirmTransaction} from '@panoptis/web3.js';
import type {
  Connection,
  Signer,
  Transaction,
  TransactionSignature,
} from '@panoptis/web3.js';

export function sendAndConfirmTransaction(
  title: string,
  connection: Connection,
  transaction: Transaction,
  ...signers: Array<Signer>
): Promise<TransactionSignature> {
  return realSendAndConfirmTransaction(connection, transaction, signers, {
    skipPreflight: false,
  });
}
