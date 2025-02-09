const transactionCheckRequests = new Map<
  number,
  {transaction: string; fixedTransaction: string; sendResponse: (response?: any) => void}
>();
let nextTransactionCheckRequestId = 0;

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log('Message', message);
  if (message.checkTransaction) {
    const id = nextTransactionCheckRequestId++;
    const {transaction, account, chain} = message.checkTransaction;
    // TODO: Fake fixing just to make it rejected
    const fixedTransaction = transaction.replace(/0x[a-fA-F0-9]{64}/g, '0x0');
    chrome.windows.create({
      url: `index.html#/transaction?id=${id}&transaction=${transaction}&account=${account}&chain=${chain}&origin=${sender.tab.id}`,
      type: 'popup',
      width: 410,
      height: 320,
      top: 100,
      left: 100,
    });
    transactionCheckRequests.set(id, {transaction, fixedTransaction, sendResponse});

    return true;
  }
  if (message.fix) {
    const request = transactionCheckRequests.get(message.fix.id);
    if (request) {
      request.sendResponse({action: 'fix', transaction: request.fixedTransaction});
      transactionCheckRequests.delete(message.fix.id);
    }
  }
  if (message.cancel) {
    const request = transactionCheckRequests.get(message.cancel.id);
    if (request) {
      request.sendResponse({action: 'cancel'});
      transactionCheckRequests.delete(message.cancel.id);
    }
  }
  if (message.proceed) {
    const request = transactionCheckRequests.get(message.proceed.id);
    if (request) {
      request.sendResponse({action: 'proceed'});
      transactionCheckRequests.delete(message.proceed.id);
    }
  }
});
