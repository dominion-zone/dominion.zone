(async function main() {
  window.addEventListener('message', event => {
    if (
      event.source !== window ||
      !event.data ||
      event.data.source !== 'dominion-page-script'
    ) {
      return;
    }

    if (event.data.checkTransaction) {
      console.log('!!!!!!!Check Transaction', event.data.checkTransaction);
      chrome.runtime.sendMessage(
        {
          checkTransaction: event.data.checkTransaction,
        },
        response => {
          console.log('!!!!!!!Response', response);
          window.postMessage({
            checkTransactionResponse: {
              ...response,
              id: event.data.checkTransaction.id,
            },
            source: 'content-script',
          });
        },
      );
      return;
    }

    if (event.data.registeredWallets) {
      chrome.runtime.sendMessage({
        registeredWallets: event.data.registeredWallets,
      });
      return;
    }

    if (event.data.unregisteredWallets) {
      chrome.runtime.sendMessage({
        unregisteredWallets: event.data.unregisteredWallets,
      });
      return;
    }
  });

  const script = document.createElement('script');
  const url = chrome.runtime.getURL('dominion.js');
  script.setAttribute('src', url);
  script.setAttribute('type', 'module');
  const container = document.head || document.documentElement;
  container.insertBefore(script, container.firstElementChild);
  container.removeChild(script);
})();
