import {Component, createSignal} from 'solid-js';
import {render} from 'solid-js/web';
import {
  ConnectSuiButton,
  SuiNetworkSelect,
  SuiWalletSelect,
  SuiWallet,
} from '../lib';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

const Test: Component = () => {
  const [netowrk, setNetwork] = createSignal('mainnet');
  const [wallet, setWallet] = createSignal<SuiWallet | null>(null);
  const preferredWallets = [
    {name: 'Sui wallet', url: 'https://suiwallet.com/'},
  ];
  return (
    <div>
      <ConnectSuiButton
        wallet={null}
        user={null}
        status="disconnected"
        connect={() => {
          console.log('connect');
        }}
        disconnect={() => console.log('disconnect')}
      />
      <SuiNetworkSelect
        network={netowrk()}
        setNetwork={setNetwork}
        networks={['mainnet', 'testnet']}
      />
      <SuiWalletSelect
        wallet={wallet()}
        setWallet={setWallet}
        preferredWallets={preferredWallets}
      />
    </div>
  );
};

render(() => <Test />, root!);
