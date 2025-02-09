import {
  Transaction,
  TransactionArgument,
  TransactionObjectInput,
} from '@mysten/sui/transactions';
import {SUI_CLOCK_OBJECT_ID} from '@mysten/sui/utils';

export type ScamtestData = {
  dataType: 'moveObject';
  fields: ScamtestFields;
  hasPublicTransfer: false;
  type: string;
};

export type ScamtestFields = {
  id: {
    id: string;
  };
  treasury_cap: {
    type: string;
    fields: {
      id: {
        id: string;
      };
      total_supply: {
        fields: {
          value: string;
        };
        type: string;
      };
    };
  };
};

export type Result = {
  $kind: 'Result';
  Result: number;
};

export type NestedResults = {
  $kind: 'NestedResult';
  NestedResult: [number, number];
}[];

// === Entry functions ===

export const callPlaceBetTo = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
  bet,
  secret,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
  bet: TransactionArgument;
  secret: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest), bet, secret, tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'place_bet_to',
  });
};

export const callMintTstTo = ({
  tx,
  packageId,
  treasuryCap,
}: {
  tx: Transaction;
  packageId: string;
  treasuryCap: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    arguments: [tx.object(treasuryCap)],
    package: packageId,
    module: 'tst',
    function: 'mint_to',
  });
};

export const callBurnWin = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
  tokens,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
  tokens: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest), tx.object(tokens)],
    package: packageId,
    module: 'scamtest',
    function: 'burn',
  });
};

export const callBurnTst = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  treasuryCap,
  tokens,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  treasuryCap: TransactionObjectInput;
  tokens: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(treasuryCap), tx.object(tokens)],
    package: packageId,
    module: 'tst',
    function: 'burn',
  });
};

export const callCleanupSlots = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest), tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'cleanup_slots',
  });
};

export const callNewOperatorOwned = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  adminCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(adminCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'new_operator_owned',
  });
};

export const callBlacklistOperator = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  adminCap,
  scamtest,
  operatorId,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
  operatorId: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(adminCap), tx.object(scamtest), operatorId],
    package: packageId,
    module: 'scamtest',
    function: 'blacklist_operator',
  });
};

export const callDestroyOperator = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  operatorCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(operatorCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'destroy_operator',
  });
};

export const callAddSlot = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  operatorCap,
  scamtest,
  slot,
  timeout,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
  slot: TransactionArgument;
  timeout: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [
      tx.object(operatorCap),
      tx.object(scamtest),
      slot,
      timeout,
      tx.object.clock(),
    ],
    package: packageId,
    module: 'scamtest',
    function: 'add_slot',
  });
};

export const callRemoveSlot = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  operatorCap,
  scamtest,
  slot,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
  slot: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [
      tx.object(operatorCap),
      tx.object(scamtest),
      slot,
      tx.object.clock(),
    ],
    package: packageId,
    module: 'scamtest',
    function: 'remove_slot',
  });
};

export const callResetSlots = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  operatorCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(operatorCap), tx.object(scamtest), tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'reset_slots',
  });
};

// === Public Functions ===

export const callPlaceBetBalance = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
  bet,
  secret,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
  bet: TransactionArgument;
  secret: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest), bet, secret, tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'place_bet_balance',
  });
};

export const callMintTstBalance = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'tst',
    function: 'mint_balance',
  });
};

export const callBurnWinBalance = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
  balance,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
  balance: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest), balance],
    package: packageId,
    module: 'scamtest',
    function: 'burn_balance',
  });
};

export const callBurnTstBalance = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
  balance,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
  balance: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest), balance],
    package: packageId,
    module: 'tst',
    function: 'burn_balance',
  });
};

// === View Functions ===

export const callAdminCapId = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'admin_cap_id',
  });
};

export const callSupplyValue = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'supply_value',
  });
};

export const callScammedValue = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'scammed_value',
  });
};

export const callAdminCapScamtestId = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  adminCap,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  adminCap: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(adminCap)],
    package: packageId,
    module: 'scamtest',
    function: 'admin_cap_scamtest_id',
  });
};

export const callOperatorCapScamtestId = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  operatorCap,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  operatorCap: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(operatorCap)],
    package: packageId,
    module: 'scamtest',
    function: 'operator_cap_scamtest_id',
  });
};

// === Admin Functions ===

export const callNew = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  supply,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  supply: TransactionArgument;
}): NestedResults => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [supply],
    package: packageId,
    module: 'scamtest',
    function: 'new',
  });
};

export const callShare = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  scamtest: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [scamtest],
    package: packageId,
    module: 'scamtest',
    function: 'share',
  });
};

export const callAssertAdminCap = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  adminCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(adminCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'assert_admin_cap',
  });
};

export const callNewOperator = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  adminCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(adminCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'new_operator',
  });
};

// === Operator Functions ===

export const callAssertOperatorCap = ({
  tx,
  packageId,
  inputCoin,
  outputCoin,
  operatorCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  inputCoin: string;
  outputCoin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [inputCoin, outputCoin],
    arguments: [tx.object(operatorCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'assert_operator_cap',
  });
};
