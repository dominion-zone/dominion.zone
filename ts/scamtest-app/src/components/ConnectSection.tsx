import {useSuiWallet, useSuiWalletController} from '@dominion.zone/solid-sui';
import {Show} from 'solid-js';
import {Check, Ellipsis} from 'lucide-solid';

const ConnectSection = () => {
  const wallet = useSuiWallet();
  const controller = useSuiWalletController();

  return (
    <section class="card">
      <span>Choose and connect your wallet using the top bar</span>
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
