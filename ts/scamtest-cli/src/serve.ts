import {Command} from 'commander';
import express from 'express';
import {keccak_256} from '@noble/hashes/sha3';
import {randomBytes} from 'crypto';
import {Transaction} from '@mysten/sui/transactions';
import {callAddSlot} from '@dominion.zone/scamtest-sdk';
import {context} from './context';

type Slot = {
  secret: Array<number>;
  value: Array<number>;
};

export const installServeCLI = (program: Command) => {
  program
    .command('serve')
    .option('--port <number>', 'Port number', '3000')
    .option('--timeout <number>', 'Timeout in milli seconds', '6000')
    .action(async ({port, timeout}: {port: string; timeout: string}) => {
      const {client, wallet, config} = context;
      const operators = await client.getOwnedObjects({
        owner: wallet!.getPublicKey().toSuiAddress(),
        filter: {
          StructType: `${config.scamtest.package}::scamtest::OperatorCap<${config.scamtest.package}::tst::TST>`,
        },
      });

      if (operators.data.length === 0) {
        throw new Error('No operator found');
      }

      const operatorCap = operators.data[0].data!.objectId;
      const app = express();

      app.get('/slot', async (req, res) => {
        const secret = randomBytes(32);
        const value = keccak_256(secret);
        const slot = {secret: Array.from(secret), value: Array.from(value)};

        const tx = new Transaction();
        tx.setGasBudget(2000000000);
        callAddSlot({
          tx,
          coin: `${config.scamtest.package}::tst::TST`,
          packageId: config.scamtest.package,
          operatorCap,
          scamtest: config.scamtest.scamtest,
          slot: tx.pure.vector('u8', slot.value),
          timeout: tx.pure.u64(parseInt(timeout)),
        });
        const result = await client.signAndExecuteTransaction({
          signer: wallet!,
          transaction: tx,
        });
        const r = await client.waitForTransaction({digest: result.digest});
        if (r.errors) {
          res.set('Content-Type', 'text/plain');
          res.status(500);
          res.send(`Transaction error:  ${r.errors}`);
          console.log('Transaction error:', r.errors);
          return;
        }

        console.log(`Serving slot ${slot.value}`);
        res.set('Content-Type', 'text/json');
        res.send(slot.secret);
      });

      app.listen(parseInt(port));
    });
};
