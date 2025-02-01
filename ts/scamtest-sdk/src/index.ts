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

export const callPlaceBetMut = ({
  tx,
  packageId,
  coin,
  scamtest,
  bet,
  secret,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
  bet: TransactionArgument;
  secret: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), bet, secret, tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'place_bet_mut',
  });
};

export const callPlaceBetTo = ({
  tx,
  packageId,
  coin,
  scamtest,
  bet,
  secret,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
  bet: TransactionArgument;
  secret: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), bet, secret, tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'place_bet_to',
  });
};

export const callMintTo = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'mint_to',
  });
};

export const callBurn = ({
  tx,
  packageId,
  coin,
  scamtest,
  tokens,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
  tokens: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), tx.object(tokens)],
    package: packageId,
    module: 'scamtest',
    function: 'burn',
  });
};

export const callCleanupSlots = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'cleanup_slots',
  });
};

export const callNewOperatorOwned = ({
  tx,
  packageId,
  coin,
  adminCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(adminCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'new_operator_owned',
  });
};

export const callBlacklistOperator = ({
  tx,
  packageId,
  coin,
  adminCap,
  scamtest,
  operatorId,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
  operatorId: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(adminCap), tx.object(scamtest), operatorId],
    package: packageId,
    module: 'scamtest',
    function: 'blacklist_operator',
  });
};

export const callDestroyOperator = ({
  tx,
  packageId,
  coin,
  operatorCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(operatorCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'destroy_operator',
  });
};

export const callAddSlot = ({
  tx,
  packageId,
  coin,
  operatorCap,
  scamtest,
  slot,
  timeout,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
  slot: TransactionArgument;
  timeout: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
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
  coin,
  operatorCap,
  scamtest,
  slot,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
  slot: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
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
  coin,
  operatorCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(operatorCap), tx.object(scamtest), tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'reset_slots',
  });
};

// === Public Functions ===

export const callPlaceBetBalanceMut = ({
  tx,
  packageId,
  coin,
  scamtest,
  bet,
  secret,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
  bet: TransactionArgument;
  secret: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), bet, secret, tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'place_bet_balance_mut',
  });
};

export const callPlaceBetBalance = ({
  tx,
  packageId,
  coin,
  scamtest,
  bet,
  secret,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
  bet: TransactionArgument;
  secret: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), bet, secret, tx.object.clock()],
    package: packageId,
    module: 'scamtest',
    function: 'place_bet_balance',
  });
};

export const callMintBalance = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'mint_balance',
  });
};

export const callBurnBalance = ({
  tx,
  packageId,
  coin,
  scamtest,
  balance,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
  balance: TransactionArgument;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest), balance],
    package: packageId,
    module: 'scamtest',
    function: 'burn_balance',
  });
};

// === View Functions ===

export const callAdminCapId = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'admin_cap_id',
  });
};

export const callSupplyValue = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'supply_value',
  });
};

export const callScammedValue = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'scammed_value',
  });
};

export const callAdminCapScamtestId = ({
  tx,
  packageId,
  coin,
  adminCap,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  adminCap: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(adminCap)],
    package: packageId,
    module: 'scamtest',
    function: 'admin_cap_scamtest_id',
  });
};

export const callOperatorCapScamtestId = ({
  tx,
  packageId,
  coin,
  operatorCap,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  operatorCap: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
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
  coin,
  supply,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  supply: TransactionArgument;
}): NestedResults => {
  return tx.moveCall({
    typeArguments: [coin],
    arguments: [supply],
    package: packageId,
    module: 'scamtest',
    function: 'new',
  });
};

export const callShare = ({
  tx,
  packageId,
  coin,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  scamtest: TransactionArgument;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [scamtest],
    package: packageId,
    module: 'scamtest',
    function: 'share',
  });
};

export const callAssertAdminCap = ({
  tx,
  packageId,
  coin,
  adminCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(adminCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'assert_admin_cap',
  });
};

export const callNewOperator = ({
  tx,
  packageId,
  coin,
  adminCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  adminCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): Result => {
  return tx.moveCall({
    typeArguments: [coin],
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
  coin,
  operatorCap,
  scamtest,
}: {
  tx: Transaction;
  packageId: string;
  coin: string;
  operatorCap: TransactionObjectInput;
  scamtest: TransactionObjectInput;
}): void => {
  tx.moveCall({
    typeArguments: [coin],
    arguments: [tx.object(operatorCap), tx.object(scamtest)],
    package: packageId,
    module: 'scamtest',
    function: 'assert_operator_cap',
  });
};
