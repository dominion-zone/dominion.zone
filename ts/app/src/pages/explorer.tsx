import {createSignal} from 'solid-js';
import KnownPackageSelector from '../components/KnownPackageSelector';
import {
  useSuiNetwork,
  useSuiUser,
  useSuiWallet,
  useSuiWalletController,
  SuiNetworkSelect,
  SuiWalletSelect,
  ConnectSuiButton,
} from '@dominion.zone/solid-sui';

export default function Explorer() {
  const [network, setNetwork] = createSignal('devnet');
  const [packageId, setPackageId] = createSignal(null);
  return (
    <div>
      <h1>Explorer</h1>
      <article class="card">
        <SuiNetworkSelect
          network={network()}
          setNetwork={setNetwork}
          networks={['devnet', 'testnet', 'mainnet']}
        />
        <KnownPackageSelector
          network={network()}
          packageId={packageId()}
          setPackageId={setPackageId}
        />
      </article>
    </div>
  );
}
