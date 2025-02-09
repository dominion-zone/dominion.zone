import {Show} from 'solid-js';
import {Check, Ellipsis, LoaderCircle} from 'lucide-solid';
import {Button, Toast, ToastProps} from 'terracotta';
import {
  SuiWallet,
  useSuiClient,
  useSuiNetwork,
  useSuiUser,
  useSuiWallet,
  useSuiWalletController,
} from '@dominion.zone/solid-sui';
import {CoinMetadata} from '../data/CoinMetadata';
import {CoinBalance, getCoinBalance} from '../data/CoinBalance';
import {action, json, Submission, useSubmission} from '@solidjs/router';
import {Transaction} from '@mysten/sui/transactions';
import {callMintTstTo} from '@dominion.zone/scamtest-sdk';
import {SUI_TYPE_ARG} from '@mysten/sui/utils';
import {
  getFaucetHost,
  getFaucetRequestStatus,
  requestSuiFromFaucetV1,
} from '@mysten/sui/faucet';
import execTx from '../utils/execTx';
import onTxComplete from '../utils/onTxComplete';
import {SuiTransactionBlockResponse} from '@mysten/sui/client';
import {useNotifications} from '../contexts/Notifications';
import {
  ErrorNotification,
  Notification,
  TransactionSuccessNotification,
} from '../stores/notifications';
import {Dynamic} from 'solid-js/web';
import config from '../stores/config';

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

type FaucetCoinInfo = {
  amount: number;
  id: string;
  transferTxDigest: string;
};

class FaucetSuccessNotification extends Notification {
  constructor(public coins: FaucetCoinInfo[], public network: string) {
    super();
  }

  override render(props: ToastProps) {
    const coins = this.coins;
    return (
      <Toast {...props}>
        Faucet request succeeded. Received{' '}
        {coins.reduce((a, {amount}) => a + amount, 0)} SUI in{' '}
        {this.coins.length} coins
      </Toast>
    );
  }
}

class MintTstSuccessNotification extends TransactionSuccessNotification {
  constructor(
    response: SuiTransactionBlockResponse,
    network: string,
    user: string,
  ) {
    super(response, network, user);
  }

  override render(props: ToastProps) {
    const l = () => this.transactionLink();
    return (
      <Toast {...props}>
        Minted test coins: <Dynamic component={l} />
      </Toast>
    );
  }
}

const MintTestCoinSection = () => {
  const network = useSuiNetwork();
  const wallet = useSuiWallet();
  const controller = useSuiWalletController();
  const user = useSuiUser();
  const client = useSuiClient();
  /*
  const testCoin = CoinMetadata({
    get network() {
      return network.value;
    },
    get coinType() {
      return `${config[network.value].scamtest.package}::tst::TST`;
    },
  });
  */

  const tstBalance = CoinBalance({
    get network() {
      return network.value;
    },
    get coinType() {
      if (!config[network.value]) {
        return null;
      }
      return `${config[network.value].scamtest.package}::tst::TST`;
    },
    get owner() {
      return user.value;
    },
  });

  const suiBalance = CoinBalance({
    get network() {
      return network.value;
    },
    coinType: SUI_TYPE_ARG,
    get owner() {
      return user.value!;
    },
  });

  const faucet = action(
    async ({network, user}: {network: string; user: string}) => {
      const host = getFaucetHost(network as 'devnet' | 'testnet' | 'localnet');
      const {task, error} = await requestSuiFromFaucetV1({
        host,
        recipient: user,
      });

      if (error) {
        throw new Error(error);
      }

      for (;;) {
        const status = await getFaucetRequestStatus({host, taskId: task!});
        if (status.error) {
          throw new Error(status.error);
        }
        switch (status.status.status) {
          case 'INPROGRESS': {
            await sleep(1000);
            continue;
          }
          case 'DISCARDED': {
            throw new Error('Faucet request discarded');
          }
          case 'SUCCEEDED': {
            throw json(status.status.transferred_gas_objects.sent, {
              revalidate: [
                getCoinBalance.keyFor({
                  network,
                  coinType: SUI_TYPE_ARG,
                  owner: user,
                }),
              ],
            });
          }
        }
      }
    },
    {
      name: 'faucetSUI',
      onComplete: submission => {
        const notifs = useNotifications();
        if (submission.error) {
          notifs.create(new ErrorNotification(submission.error));
        } else {
          notifs.create(
            new FaucetSuccessNotification(submission.result, network.value),
          );
        }
      },
    },
  );

  const mintTst = action(
    async ({network, user, wallet}) => {
      const tx = new Transaction();
      callMintTstTo({
        tx,
        packageId: config[network].scamtest.package,
        treasuryCap: config[network].scamtest.tstCap,
      });
      const result = await execTx({
        tx,
        wallet,
        user,
        network,
        client: client(),
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
      name: 'mintTst',
      onComplete: onTxComplete<{
        network: string;
        user: string;
        wallet: SuiWallet;
      }>(
        ({network, user, response}) =>
          new MintTstSuccessNotification(response, network, user),
      ),
    },
  );

  const faucetSubmission = useSubmission(faucet);
  const mintSubmission = useSubmission(mintTst);

  return (
    <section class="card">
      <div class="card-container">
        <h2>Mint Test Tokens</h2>
        <form method="post">
          <ul>
            <li>
              Click the{' '}
              <Button
                formAction={faucet.with({
                  network: network.value,
                  user: user.value!,
                })}
                type="submit"
                disabled={
                  controller.status !== 'connected' || faucetSubmission.pending
                }
              >
                <Show when={faucetSubmission.pending}>
                  <LoaderCircle class="button-icon" />
                </Show>
                Mint SUI
              </Button>{' '}
              button (Current balance: {suiBalance()?.totalBalance ?? '0'} devnet SUI)
              to receive test SUI for transaction fees.
            </li>
            <li>
              Click the{' '}
              <Button
                formAction={mintTst.with({
                  network: network.value,
                  user: user.value!,
                  wallet: wallet.value!,
                })}
                type="submit"
                disabled={
                  controller.status !== 'connected' || mintSubmission.pending
                }
              >
                <Show when={mintSubmission.pending}>
                  <LoaderCircle class="button-icon" />
                </Show>
                Mint TST
              </Button>{' '}
              button (Current balance: {tstBalance()?.totalBalance ?? '0'} TST) to generate TST tokens, which are used for
              the scam demonstration.
            </li>
          </ul>
        </form>
      </div>
      <span class="icon">
        <Show
          when={
            BigInt(tstBalance()?.totalBalance ?? '0') > 0n &&
            BigInt(suiBalance()?.totalBalance ?? '0')
          }
          fallback={<Ellipsis />}
        >
          <Check />
        </Show>
      </span>
    </section>
  );
};

export default MintTestCoinSection;
