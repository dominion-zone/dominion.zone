import {useSuiWallet, useSuiWalletController} from '@dominion.zone/solid-sui';
import {Show} from 'solid-js';
import {Check, Ellipsis} from 'lucide-solid';

const ConnectSection = () => {
  const wallet = useSuiWallet();
  const controller = useSuiWalletController();

  return (
    <section class="card">
      <div class="card-container">
        <h2>Connect Your Wallet to Start</h2>
        <span>
          Use the <strong>top bar</strong> to select and connect your preferred
          wallet.
        </span>
      </div>
      <span class="icon">
        <Show
          when={wallet.value && controller.status === 'connected'}
          fallback={<Ellipsis />}
        >
          <Check />
        </Show>
      </span>
    </section>
  );
};

export default ConnectSection;
