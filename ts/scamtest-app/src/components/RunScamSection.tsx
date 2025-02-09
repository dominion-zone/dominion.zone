import {callPlaceBetTo} from '@dominion.zone/scamtest-sdk';
import {
  SuiWallet,
  useSuiClient,
  useSuiNetwork,
  useSuiUser,
  useSuiWallet,
  useSuiWalletController,
} from '@dominion.zone/solid-sui';
import {Transaction} from '@mysten/sui/transactions';
import {action, json, useSubmission} from '@solidjs/router';
import {Match, Show, Switch} from 'solid-js';
import {Button, Toast} from 'terracotta';
import {CoinBalance, getCoinBalance} from '../data/CoinBalance';
import axios from 'axios';
import {CoinStruct, SuiTransactionBlockResponse} from '@mysten/sui/client';
import {LoaderCircle} from 'lucide-solid';
import onTxComplete from '../utils/onTxComplete';
import execTx from '../utils/execTx';
import {SUI_TYPE_ARG} from '@mysten/sui/utils';
import {TransactionSuccessNotification} from '../stores/notifications';
import {Dynamic} from 'solid-js/web';
import * as config from '../stores/config';

class ScamNotification extends TransactionSuccessNotification {
  constructor(
    response: SuiTransactionBlockResponse,
    network: string,
    user: string,
  ) {
    super(response, network, user);
  }

  render(props: any) {
    const balance = BigInt(
      this.response.balanceChanges.find(
        ({coinType, owner}) =>
          coinType ===
            `${config[this.network].scamtest.package}::tst::TST` &&
          owner['AddressOwner'] === this.user,
      ).amount,
    );
    const l = () => this.transactionLink();

    return (
      <Toast {...props}>
        <Switch>
          <Match when={balance > 0n}> You won {balance.toString()} TST:</Match>
          <Match when={balance <= 0n}>
            {' '}
            You was scammed for {(-balance).toString()} TST:
          </Match>
        </Switch>
        {' '}<Dynamic component={l} />
      </Toast>
    );
  }
}

const RunScamSection = () => {
  const wallet = useSuiWallet();
  const controller = useSuiWalletController();
  const network = useSuiNetwork();
  const user = useSuiUser();
  const client = useSuiClient();

  const tstBalance = CoinBalance({
    get network() {
      return network.value;
    },
    get coinType() {
      return `${config[network.value].scamtest.package}::tst::TST`;
    },
    get owner() {
      return user.value!;
    },
  });

  const runScam = action(
    async ({network, user, wallet}) => {
      const coins: CoinStruct[] = [];
      let cursor = null;
      for (;;) {
        const pack = await client().getCoins({
          owner: user,
          coinType: `${config[network].scamtest.package}::tst::TST`,
          cursor,
        });
        coins.push(...pack.data);
        if (!pack.hasNextPage) {
          break;
        }
        cursor = pack.nextCursor;
      }
      if (coins.length === 0) {
        throw new Error('No coins');
      }
      const slotResponse = (await axios.get(config[network].slotUrl)).data;
      const tx = new Transaction();
      tx.setGasBudget(2000000000);
      tx.mergeCoins(
        tx.object(coins[0].coinObjectId),
        coins.slice(1).map(c => tx.object(c.coinObjectId)),
      );
      callPlaceBetTo({
        tx,
        packageId: config[network].scamtest.package,
        scamtest: config[network].scamtest.scamtest,
        inputCoin: `${config[network].scamtest.package}::tst::TST`,
        outputCoin: `${config[network].scamtest.package}::tst::Win`,
        bet: tx.object(coins[0].coinObjectId),
        secret: tx.pure.vector('u8', slotResponse),
      });
      const result = await execTx({
        tx,
        wallet,
        user,
        network,
        client: client(),
        options: {
          showBalanceChanges: true,
        },
      });
      throw json(result, {
        revalidate: [
          getCoinBalance.keyFor({
            network,
            coinType: SUI_TYPE_ARG,
            owner: user,
          }),
          getCoinBalance.keyFor({
            network,
            coinType: `${config[network].scamtest.package}::tst::TST`,
            owner: user,
          }),
        ],
      });
    },
    {
      name: 'runScam',
      onComplete: onTxComplete<{network: string; user: string; wallet: SuiWallet}>(
        ({network, user, response}) =>
          new ScamNotification(response, network, user),
      ),
    },
  );

  const runScamSubmission = useSubmission(runScam);

  return (
    <section class="card">
      <form
        action={runScam.with({network: network.value, user: user.value!, wallet: wallet.value})}
        method="post"
      >
        Run the transaction simulating it will x2 your test coins but if you
        click the approve button not quick enough it will drain your test coins
        instead{' '}
        <Button
          type="submit"
          disabled={
            controller.status !== 'connected' ||
            runScamSubmission.pending ||
            BigInt(tstBalance()?.totalBalance ?? '0') === 0n
          }
        >
          <Show when={runScamSubmission.pending}>
            <LoaderCircle class="button-icon" />
          </Show>
          Try
        </Button>
      </form>
    </section>
  );
};

export default RunScamSection;
