import {useSuiWallet, useSuiWalletController} from '@dominion.zone/solid-sui';
import {Match, Switch} from 'solid-js';
import {Check, Ellipsis, ShieldQuestion} from 'lucide-solid';

const CheckNetworkSection = () => {
  const wallet = useSuiWallet();
  const controller = useSuiWalletController();

  return (
    <section class="card">
      <span>Switch your wallet to the devnet network</span>
      <span class="icon">
        <Switch fallback={<Ellipsis />}>
          <Match when={
            wallet.value &&
            controller.status === 'connected' &&
            wallet.value.chains.find(chain => chain === 'sui:devnet') &&
            wallet.value.chains.filter(chain => chain.startsWith('sui:')).length === 1
          }>
            <Check />
          </Match>
          <Match when={
            wallet.value &&
            controller.status === 'connected' &&
            wallet.value.chains.find(chain => chain === 'sui:devnet') &&
            wallet.value.chains.filter(chain => chain.startsWith('sui:')).length > 1
          }>
            <ShieldQuestion />
          </Match>
        </Switch>
      </span>
    </section>
  );
};

export default CheckNetworkSection;