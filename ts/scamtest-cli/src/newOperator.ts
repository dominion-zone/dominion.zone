import {Command} from 'commander';
import {context} from './context';
import {Transaction} from '@mysten/sui/transactions';
import {callNewOperatorOwned} from '@dominion.zone/scamtest-sdk';

export const installNewOperatorCLI = (program: Command) => {
  program
    .command('new-operator')
    .option('--port <number>', 'Port number', '3000')
    .action(async ({port}: {port: string}) => {
      const {client, wallet, config} = context;
      const tx = new Transaction();
      tx.setGasBudget(2000000000);
      callNewOperatorOwned({
        tx,
        coin: `${config.scamtest.package}::tst::TST`,
        packageId: config.scamtest.package,
        adminCap: tx.object(config.scamtest.adminCap),
        scamtest: tx.object(config.scamtest.scamtest),
      });
      const result = await client.signAndExecuteTransaction({
        signer: wallet!,
        transaction: tx,
      });
      const r = await client.waitForTransaction({digest: result.digest});
      if (r.errors) {
        console.log(`Tx ${result.digest} error`);
        console.error(r.errors);
        return;
      }
      console.log(`Created operator: ${result.digest}`);
    });
};
