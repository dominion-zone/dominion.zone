import {
  ConnectSuiButton,
  useSuiNetwork,
  useSuiUser,
  useSuiWallet,
  useSuiWalletController,
} from '@dominion.zone/solid-sui';
import styles from '../styles/index.module.css';
import ConnectSection from '../components/ConnectSection';
import CheckNetworkSection from '../components/CheckNetworkSection';
import MintTestCoinSection from '../components/MintTestCoinSection';
import {RoutePreloadFuncArgs} from '@solidjs/router';
import {getCoinMetadata} from '../data/CoinMetadata';
import {getCoinBalance} from '../data/CoinBalance';
import RunScamSection from '../components/RunScamSection';
import * as config from '../stores/config';

export default function Home() {
  const network = useSuiNetwork();
  const wallet = useSuiWallet();
  const user = useSuiUser();
  const walletController = useSuiWalletController();

  return (
    <main>
      <h2 class={styles.title}>Testing your SUI wallet for the drain attack</h2>
      <ul>
        <li>
          <ConnectSection />
        </li>
        <li>
          <CheckNetworkSection />
        </li>
        <li>
          <MintTestCoinSection />
        </li>
        <li>
          <RunScamSection />
        </li>
      </ul>
    </main>
  );
}

Home.routePreload = ({location}: RoutePreloadFuncArgs) => {
  const network = location.query.network as string;
  if (config[network]?.scamtest) {
    const coinType = `${config[network].scamtest.package}::tst::TST`;
    void getCoinMetadata({
      network,
      coinType,
    });
    if (location.query.user) {
      void getCoinBalance({
        network,
        coinType,
        owner: location.query.user as string,
      });
    }
  }
};
