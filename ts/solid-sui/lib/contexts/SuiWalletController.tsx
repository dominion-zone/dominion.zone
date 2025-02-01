import {
  Component,
  createContext,
  createEffect,
  createSignal,
  ParentProps,
  Setter,
  useContext,
} from 'solid-js';
import {SuiWallet} from './SuiWallet';
import {StandardConnect, StandardDisconnect} from '@mysten/wallet-standard';

export const SuiWalletDisconnected = 'disconnected';
export const SuiWalletConnecting = 'connecting';
export const SuiWalletConnected = 'connected';

export type SuiWalletConnectionStatus =
  | typeof SuiWalletDisconnected
  | typeof SuiWalletConnecting
  | typeof SuiWalletConnected;

export type SuiWalletControllerConext = {
  status: SuiWalletConnectionStatus;
  connect: () => void;
  disconnect: () => void;
};

const SuiWalletControllerConext = createContext<SuiWalletControllerConext>();

export type SuiWalletControllerContextProviderProps = ParentProps<{
  autoConnect?: boolean;
  wallet: SuiWallet | null;
  user: string | null;
  setUser: Setter<string | null>;
  network: string;
  setNetwork: Setter<string>;
}>;

export const SuiWalletControllerProvider: Component<
  SuiWalletControllerContextProviderProps
> = props => {
  const [status, setStatus] = createSignal<SuiWalletConnectionStatus>(
    SuiWalletDisconnected,
  );

  createEffect(oldChains => {
    if (!props.wallet || oldChains === props.wallet.chains) {
      return;
    }
    if (!props.wallet.chains.includes(`sui:${props.network}`)) {
      const suiNetwork = props.wallet.chains.find(c => c.startsWith('sui:'));
      if (suiNetwork) {
        props.setNetwork(suiNetwork.split(':')[1]);
      }
    }
    return props.wallet.chains;
  }, props.wallet?.chains);

  createEffect(oldAccounts => {
    if (!props.user || !props.wallet || oldAccounts === props.wallet.accounts) {
      return;
    }
    if (
      props.wallet.accounts.every(a => a.address !== props.user) &&
      props.wallet.accounts.length > 0 &&
      props.wallet.chains.find(c => c.startsWith('sui:'))
    ) {
      props.setUser(props.wallet.accounts[0].address);
    }
  }, props.wallet?.accounts);

  const connect = () => {
    if (!props.wallet) {
      return;
    }
    setStatus(SuiWalletConnecting);
    props.wallet.features[StandardConnect].connect()
      .then(({accounts}) => {
        if (
          accounts &&
          (!props.user || accounts.every(a => a.address !== props.user))
        ) {
          props.setUser(accounts[0].address);
        }
        setStatus(SuiWalletConnected);
      })
      .catch(() => {
        setStatus(SuiWalletDisconnected);
      });
  };

  const disconnect = () => {
    if (!props.wallet) {
      return;
    }
    if (status() !== SuiWalletDisconnected) {
      const disconnect = props.wallet.features[StandardDisconnect];
      if (disconnect) {
        disconnect.disconnect().finally(() => {
          setStatus(SuiWalletDisconnected);
        });
      } else {
        setStatus(SuiWalletDisconnected);
      }
    }
  };

  createEffect<SuiWallet | null>(prevWallet => {
    if (prevWallet?.id === props.wallet?.id) {
      return props.wallet;
    }

    if (prevWallet) {
      if (status() !== SuiWalletDisconnected) {
        const disconnect = prevWallet.features[StandardDisconnect];
        if (disconnect) {
          disconnect.disconnect().finally(() => {
            setStatus(SuiWalletDisconnected);
            if (props.wallet && props.autoConnect) {
              connect();
            }
          });
        } else {
          setStatus(SuiWalletDisconnected);
          if (props.wallet && props.autoConnect) {
            connect();
          }
        }
      }
    }

    return props.wallet;
  }, null);

  return (
    <SuiWalletControllerConext.Provider
      value={{
        get status() {
          return status();
        },
        connect,
        disconnect,
      }}
    >
      {props.children}
    </SuiWalletControllerConext.Provider>
  );
};

export const useSuiWalletController = (): SuiWalletControllerConext => {
  const controller = useContext(SuiWalletControllerConext);
  if (!controller) {
    throw new Error(
      'useSuiWalletController must be used within a SuiWalletControllerProvider',
    );
  }
  return controller;
};
