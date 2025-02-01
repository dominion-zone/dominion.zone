import {SuiTransactionBlockResponse} from '@mysten/sui/client';
import {formatDigest} from '@mysten/sui/utils';
import {JSX} from 'solid-js';
import {Dynamic} from 'solid-js/web';
import {Toast, ToasterStore, ToastProps} from 'terracotta';

export abstract class Notification {
  abstract render(props: ToastProps): JSX.Element;
}

export class ErrorNotification extends Notification {
  constructor(public readonly error: Error) {
    super();
  }

  override render(props: ToastProps) {
    const error = this.error;
    return <Toast {...props}>{error.message}</Toast>;
  }
}

export class TransactionSuccessNotification extends Notification {
  constructor(
    public readonly response: SuiTransactionBlockResponse,
    public readonly network: string,
    public readonly user: string,
  ) {
    super();
  }

  transactionLink() {
    const digest = this.response.digest;
    const network = this.network;
    return (
      <a
        target="_blank"
        rel="noreferrer"
        href={`https://${
          network === 'mainnet' ? '' : network + '.'
        }suivision.xyz/txblock/${digest}`}
      >
        {formatDigest(digest)}
      </a>
    );
  }

  override render(props: ToastProps) {
    console.log('TransactionSuccessNotification render');
    const l = () => this.transactionLink();
    return (
      <Toast {...props}>
        Tx <Dynamic component={l} /> success
      </Toast>
    );
  }
}

export const notifications = new ToasterStore<Notification>();
