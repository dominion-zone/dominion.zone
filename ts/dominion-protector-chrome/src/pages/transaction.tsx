import {Button} from 'terracotta';
import styles from '../styles/transaction.module.css';
import {action, useSearchParams} from '@solidjs/router';

export default function Transaction() {
  const [searchParams] = useSearchParams<{
    id: string;
    transaction: string;
    account: string;
    chain: string;
    origin: string;
  }>();
  const tx = JSON.parse(searchParams.transaction);
  console.log(tx);

  const fix = action(async (id: number) => {
    await chrome.runtime.sendMessage({
      fix: {
        id,
      },
    });
    window.close();
  });

  const cancel = action(async (id: number) => {
    await chrome.runtime.sendMessage({
      cancel: {
        id,
      },
    });
    window.close();
  });

  const proceed = action(async (id: number) => {
    await chrome.runtime.sendMessage({
      proceed: {
        id,
      },
    });
    window.close();
  });
  return (
    <>
      <h2>⚠ Warning: Suspicious Transaction</h2>
      <p>
        This blockchain transaction may be harmful. Please review the details
        before proceeding.
      </p>
      <form method="post" class={styles.buttons}>
        <Button
          type="submit"
          class={styles.fix}
          formAction={fix.with(parseInt(searchParams.id))}
        >
          Fix Transaction
        </Button>
        <Button
          type="submit"
          class={styles.cancel}
          formAction={cancel.with(parseInt(searchParams.id))}
        >
          Cancel
        </Button>
        <Button
          type="submit"
          class={styles.proceed}
          formAction={proceed.with(parseInt(searchParams.id))}
        >
          Proceed Anyway
        </Button>
      </form>
    </>
  );
}
