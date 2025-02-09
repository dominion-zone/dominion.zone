import {
  getWallets,
  isWalletWithRequiredFeatureSet,
  registerWallet,
  SignedTransaction,
  StandardConnect,
  StandardConnectInput,
  StandardConnectOutput,
  StandardEvents,
  SuiSignAndExecuteTransactionFeature,
  SuiSignAndExecuteTransactionInput,
  SuiSignAndExecuteTransactionOutput,
  SuiSignPersonalMessageInput,
  SuiSignPersonalMessageOutput,
  SuiSignTransactionFeature,
  SuiSignTransactionInput,
  Wallet,
  WalletEventsWindow,
  WalletWithRequiredFeatures,
  WalletWithStandardFeatures,
  WalletWithSuiFeatures,
} from '@mysten/wallet-standard';

const LOGO_BASE64 =
  'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjQiIGhlaWdodD0iNjQiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PHJlY3Qgd2lkdGg9IjY0IiBoZWlnaHQ9IjY0IiByeD0iMjQiIGZpbGw9InVybCgjcGFpbnQwX3JhZGlhbF8zMDVfMTI1MTYpIi8+PHBhdGggZD0iTTUxLjUgNDMuNmMtMy45IDAtNy42LTMuOS05LjUtNi40LTEuOSAyLjUtNS42IDYuNC05LjUgNi40LTQgMC03LjctMy45LTkuNS02LjQtMS44IDIuNS01LjUgNi40LTkuNSA2LjQtLjggMC0xLjUtLjYtMS41LTEuNSAwLS44LjctMS41IDEuNS0xLjUgMy4yIDAgNy4xLTUuMSA4LjItNi45LjMtLjQuOC0uNyAxLjMtLjdzMSAuMiAxLjMuN2MxLjEgMS44IDUgNi45IDguMiA2LjkgMy4xIDAgNy4xLTUuMSA4LjItNi45LjMtLjQuOC0uNyAxLjMtLjdzMSAuMiAxLjIuN2MxLjEgMS44IDUgNi45IDguMiA2LjkuOSAwIDEuNi43IDEuNiAxLjUgMCAuOS0uNiAxLjUtMS41IDEuNXoiIGZpbGw9IiNmZmYiLz48cGF0aCBkPSJNNTEuNSA1Mi4zYy0zLjkgMC03LjYtMy45LTkuNS02LjQtMS45IDIuNS01LjYgNi40LTkuNSA2LjQtNCAwLTcuNy0zLjktOS41LTYuNC0xLjggMi41LTUuNSA2LjQtOS41IDYuNC0uOCAwLTEuNS0uNi0xLjUtMS41IDAtLjguNy0xLjUgMS41LTEuNSAzLjIgMCA3LjEtNS4xIDguMi02LjkuMy0uNC44LS43IDEuMy0uN3MxIC4zIDEuMy43YzEuMSAxLjggNSA2LjkgOC4yIDYuOSAzLjEgMCA3LjEtNS4xIDguMi02LjkuMy0uNC44LS43IDEuMy0uN3MxIC4zIDEuMi43YzEuMSAxLjggNSA2LjkgOC4yIDYuOS45IDAgMS42LjcgMS42IDEuNSAwIC45LS42IDEuNS0xLjUgMS41ek0xNC42IDM2LjdjLS44IDAtMS40LS41LTEuNi0xLjNsLS4zLTMuNmMwLTEwLjkgOC45LTE5LjggMTkuOC0xOS44IDExIDAgMTkuOCA4LjkgMTkuOCAxOS44bC0uMyAzLjZjLS4xLjgtLjkgMS40LTEuNyAxLjItLjktLjEtMS41LS45LTEuMy0xLjhsLjMtM2MwLTkuMi03LjUtMTYuOC0xNi44LTE2LjgtOS4yIDAtMTYuNyA3LjUtMTYuNyAxNi44bC4yIDMuMWMuMi44LS4zIDEuNi0xLjEgMS44aC0uM3oiIGZpbGw9IiNmZmYiLz48ZGVmcz48cmFkaWFsR3JhZGllbnQgaWQ9InBhaW50MF9yYWRpYWxfMzA1XzEyNTE2IiBjeD0iMCIgY3k9IjAiIHI9IjEiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBncmFkaWVudFRyYW5zZm9ybT0ibWF0cml4KDUyLjc1ODAzIDUxLjM1OCAtNTEuNDM5NDcgNTIuODQxNzIgMCA3LjQwNykiPjxzdG9wIHN0b3AtY29sb3I9IiMwMDU4REQiLz48c3RvcCBvZmZzZXQ9IjEiIHN0b3AtY29sb3I9IiM2N0M4RkYiLz48L3JhZGlhbEdyYWRpZW50PjwvZGVmcz48L3N2Zz4=';

const checkTransactionRequests = new Map<
  number,
  {
    tx: string;
    resolve: (value: string | PromiseLike<string>) => void;
    reject: (reason?: any) => void;
    signal?: AbortSignal;
  }
>();
let nextTransactionCheckRequestId = 0;

window.addEventListener('message', event => {
  if (
    event.source !== window ||
    !event.data ||
    event.data.source !== 'content-script'
  ) {
    return;
  }
  if (event.data.checkTransactionResponse) {
    console.log('Response', event.data.checkTransactionResponse);
    const {tx, signal, resolve, reject} = checkTransactionRequests.get(
      event.data.checkTransactionResponse.id,
    );
    checkTransactionRequests.delete(event.data.checkTransactionResponse.id);
    if (signal?.aborted) {
      return reject(new Error(signal.reason));
    }
    switch (event.data.checkTransactionResponse.action) {
      case 'fix':
        return resolve(event.data.checkTransactionResponse.transaction);
      case 'cancel':
        return reject(
          new Error('Dominion protection has canceled the transaction'),
        );
      case 'proceed':
        return resolve(tx);
      default:
        return reject(new Error('Internal error'));
    }
  }
});

const checkTransaction = ({
  tx,
  account,
  chain,
  signal,
}: Omit<SuiSignTransactionInput, 'transaction'> & {
  tx: string;
}): Promise<string> => {
  const id = nextTransactionCheckRequestId++;
  return new Promise((resolve, reject) => {
    checkTransactionRequests.set(id, {tx, resolve, reject, signal});
    console.log(`Request ${id}`);

    window.postMessage({
      checkTransaction: {
        id,
        transaction: tx,
        account,
        chain,
      },
      source: 'dominion-page-script',
    });
  });
};

function installProtector(origin: Wallet): Wallet | null {
  if (
    !isWalletWithRequiredFeatureSet(origin) ||
    !origin.chains.some(chain => chain.split(':')[0] === 'sui')
  ) {
    return null;
  }
  const id = `DP:${origin.id || origin.name}`;
  const name = `DP: ${origin.name}`;

  const features = {
    ...origin.features,
  };

  if (origin.features['sui:signTransaction']) {
    const originFeatures =
      origin.features as unknown as SuiSignTransactionFeature;
    if (originFeatures['sui:signTransaction'].version !== '2.0.0') {
      throw new Error(
        'Unsupported version of the sui:signTransaction feature. Expected version 2.0.0.',
      );
    }
    const signTransaction = async ({
      transaction,
      account,
      chain,
      signal,
    }: SuiSignTransactionInput): Promise<SignedTransaction> => {
      let tx = await transaction.toJSON();
      tx = await checkTransaction({tx, account, chain, signal});

      return await originFeatures['sui:signTransaction'].signTransaction({
        transaction: {...transaction, toJSON: async () => tx},
        account,
        chain,
        signal,
      });
    };

    (features as unknown as SuiSignTransactionFeature)['sui:signTransaction'] =
      {
        version: '2.0.0',
        signTransaction,
      };
  }

  if (origin.features['sui:signAndExecuteTransaction']) {
    const originFeatures =
      origin.features as unknown as SuiSignAndExecuteTransactionFeature;
    if (originFeatures['sui:signAndExecuteTransaction'].version !== '2.0.0') {
      throw new Error(
        'Unsupported version of the sui:signAndExecuteTransaction feature. Expected version 2.0.0.',
      );
    }

    const signAndExecuteTransaction = async ({
      transaction,
      account,
      chain,
      signal,
    }: SuiSignAndExecuteTransactionInput): Promise<SuiSignAndExecuteTransactionOutput> => {
      let tx = await transaction.toJSON();
      tx = await checkTransaction({tx, account, chain, signal});

      return await originFeatures[
        'sui:signAndExecuteTransaction'
      ].signAndExecuteTransaction({
        transaction: {...transaction, toJSON: async () => tx},
        account,
        chain,
        signal,
      });
    };

    (features as unknown as SuiSignAndExecuteTransactionFeature)[
      'sui:signAndExecuteTransaction'
    ] = {
      version: '2.0.0',
      signAndExecuteTransaction,
    };
  }

  const wallet: WalletWithRequiredFeatures = Object.freeze({
    version: '1.0.0',
    id,
    name,
    icon: LOGO_BASE64,
    chains: origin.chains,
    features,
    accounts: [],
  });
  registerWallet(wallet);
  return wallet;
}

async function trackTabWallets() {
  const registeredWalletsSet = new Set<Wallet>();

  const register = (...wallets: Wallet[]) => {
    // Filter out wallets that have already been registered.
    // This prevents the same wallet from being registered twice, but it also prevents wallets from being
    // unregistered by reusing a reference to the wallet to obtain the unregister function for it.
    wallets = wallets.filter(
      wallet =>
        !registeredWalletsSet.has(wallet) && !wallet?.id?.startsWith('DP:'),
    );
    // If there are no new wallets to register, just return a no-op unregister function.
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    if (!wallets.length) return () => {};

    let protectors = wallets
      .map(wallet => {
        registeredWalletsSet.add(wallet);
        return installProtector(wallet);
      })
      .filter(protector => protector);
    window.postMessage({
      registeredWallets: wallets.map(
        ({id, name, icon, chains, features, accounts}) => ({
          id,
          name,
          icon,
          chains,
          features: Object.keys(features),
          accounts,
        }),
      ),
      source: 'dominion-page-script',
    });
    // Return a function that unregisters the registered wallets.
    return function unregister(): void {
      wallets.forEach(wallet => registeredWalletsSet.delete(wallet));
      window.postMessage({
        unregisteredWallets: wallets.map(({id, name}) => ({
          id,
          name,
        })),
        source: 'dominion-page-script',
      });
    };
  };

  const wallets = getWallets();
  register(...wallets.get());
  wallets.on('register', register);
}

(async function main() {
  trackTabWallets();
})();
