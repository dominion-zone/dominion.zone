import {createEffect, createSignal} from 'solid-js';
import KnownPackageSelector from '../components/KnownPackageSelector';
import {
  useSuiNetwork,
  useSuiUser,
  useSuiWallet,
  useSuiWalletController,
  SuiNetworkSelect,
  SuiWalletSelect,
  ConnectSuiButton,
  SuiNetworkProvider,
  SuiClientProvider,
} from '@dominion.zone/solid-sui';
import {Button} from 'terracotta';
import {useSearchParams} from '@solidjs/router';
import PackageDescriptionView from '../components/PackageDescriptionView';
import {isValidSuiAddress} from '@mysten/sui/utils';

export default function Explorer() {
  const [searchParams, setSearchParams] = useSearchParams();
  const [network, setNetwork] = createSignal(searchParams.network as string);
  const [packageId, setPackageId] = createSignal(searchParams.packageId as string);
  createEffect(() => {
    setSearchParams({network: network()});
  });
  createEffect(() => {
    if (isValidSuiAddress(packageId())) {
      setSearchParams({packageId: packageId()});
    }
  });
  return (
    <div>
      <h1>Explorer</h1>
      <section class="card">
        <div>
          <span>Network:</span>
          <SuiNetworkSelect
            network={network()}
            setNetwork={setNetwork}
            networks={['devnet', 'testnet', 'mainnet']}
            class="network-select"
          />
        </div>
        <div>
          <span>Package</span>
          <KnownPackageSelector
            network={network()}
            packageId={packageId()}
            setPackageId={setPackageId}
            class="package-select"
          />
        </div>
      </section>
      <PackageDescriptionView
        network={searchParams.network as string}
        packageId={searchParams.packageId as string}
      />
    </div>
  );
}
