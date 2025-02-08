import {
  useSuiNetwork,
  useSuiUser,
  useSuiWallet,
  useSuiWalletController,
  SuiNetworkSelect,
  SuiWalletSelect,
  ConnectSuiButton,
} from '@dominion.zone/solid-sui';

import styles from '../styles/AppBar.module.css';
import {Show} from 'solid-js';

const AppBar = () => {
  const preferredWallets = [
    {name: 'Sui Wallet', url: 'https://suiwallet.com/'},
  ];

  const network = useSuiNetwork();
  const wallet = useSuiWallet();
  const walletController = useSuiWalletController();
  const user = useSuiUser();

  /*
  const [menuOpen, setMenuOpen] = createSignal(false);
  const toggleMenu = () => {
    setMenuOpen(v => !v);
  };
  */

  const networks = () => Object.keys(window.CONFIG);

  return (
    <header class={styles.header}>
      <div class={styles.headerContainer}>
        <div class={styles.logo}>
          <img class={styles.logoIcon} src="./scamtest.png" />
          <span>Fake scam</span>
        </div>
        {/*
        <div class={styles.menuToggle} onclick={toggleMenu}>
          <SquareMenu />
        </div>
        */}
        <nav
          classList={{
            [styles.navControls]: true,
            // [styles.active]: menuOpen(),
          }}
        >
          <Show when={networks().length > 1}>
            <SuiNetworkSelect
              networks={networks()}
              network={network.value}
              setNetwork={network.set!}
            />
          </Show>
          <SuiWalletSelect
            preferredWallets={preferredWallets}
            wallet={wallet.value}
            setWallet={wallet.set!}
          />
          <ConnectSuiButton
            class={styles.connectButton}
            wallet={wallet.value}
            user={user.value}
            {...walletController}
          />
        </nav>
      </div>
    </header>
  );
};

export default AppBar;
