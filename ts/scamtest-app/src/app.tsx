import {
  createEffect,
  createSignal,
  Setter,
  Suspense,
  type Component,
} from 'solid-js';
import {RouteSectionProps, useSearchParams} from '@solidjs/router';
import {makePersisted} from '@solid-primitives/storage';
import {
  SuiAutoconnectProvider,
  SuiClientFactoryProvider,
  SuiNetworkProvider,
  SuiUserProvider,
  SuiWallet,
  SuiWalletControllerProvider,
  SuiWalletProvider,
  wallets,
} from '@dominion.zone/solid-sui';
import {isWalletWithRequiredFeatureSet} from '@mysten/wallet-standard';
import AppBar from './components/AppBar';
import {setConfigs, suiClientCache} from './stores/suiClient';
import AppToaster from './components/AppToaster';

const App: Component<RouteSectionProps> = props => {
  const [searchParams, setSearchParams] = useSearchParams<{
    network?: string;
    user?: string;
  }>();

  createEffect(() => {
    if (!searchParams.network) {
      setSearchParams({network: 'devnet'});
    }
  });

  const setSuiNetwork = ((value: string | ((prev: string) => string)) => {
    if (typeof value === 'function') {
      value = value(props.params.network);
    }
    setSearchParams({network: value});
    return value;
  }) as Setter<string>;

  const setSuiUser = (value => {
    const v =
      typeof value === 'function' ? value(searchParams.user ?? null) : value;
    setSearchParams({user: v});
    return value;
  }) as Setter<string | null>;

  const [suiWallet, setSuiWallet] = makePersisted(
    createSignal<SuiWallet | null>(null),
    {
      serialize(w: SuiWallet | null) {
        return w?.id ?? '';
      },
      deserialize(id: string) {
        return (
          (wallets.find(
            w =>
              w.id === id &&
              isWalletWithRequiredFeatureSet(w) &&
              w.chains.some(chain => chain.split(':')[0] === 'sui'),
          ) as SuiWallet) ?? null
        );
      },
    },
  );

  const [suiAutoconnect, setSuiAutoconnect] = makePersisted(
    createSignal(false),
  );

  const setSuiNetworkChecked = ((value: string | ((prev: string) => string)) =>
    setSuiNetwork(prev => {
      if (typeof value === 'function') {
        value = value(prev);
      }
      if (window.CONFIG[value]) {
        return value;
      } else {
        return prev;
      }
    })) as Setter<string>;

  return (
    <SuiUserProvider value={searchParams.user ?? null} set={setSuiUser}>
      <SuiNetworkProvider
        value={searchParams.network ?? 'devnet'}
        set={setSuiNetwork}
      >
        <SuiClientFactoryProvider
          value={suiClientCache}
          set={setConfigs}
          netrwork={searchParams.network ?? 'devnet'}
        >
          <SuiWalletProvider value={suiWallet()} set={setSuiWallet}>
            <SuiAutoconnectProvider
              value={suiAutoconnect()}
              set={setSuiAutoconnect}
            >
              <SuiWalletControllerProvider
                wallet={suiWallet()}
                autoConnect={suiAutoconnect()}
                user={searchParams.user ?? null}
                setUser={setSuiUser}
                network={searchParams.network ?? 'devnet'}
                setNetwork={setSuiNetworkChecked}
              >
                <AppBar />
                <Suspense>{props.children}</Suspense>
                <AppToaster />
              </SuiWalletControllerProvider>
            </SuiAutoconnectProvider>
          </SuiWalletProvider>
        </SuiClientFactoryProvider>
      </SuiNetworkProvider>
    </SuiUserProvider>
  );
};

export default App;
