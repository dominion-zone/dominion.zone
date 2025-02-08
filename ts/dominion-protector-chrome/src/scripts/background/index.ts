const transactionCheckRequests = new Map<number, (response?: any) => void>();
let nextTransactionCheckRequestId = 0;

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log('Message', message);
  if (message.checkTransaction) {
    const id = nextTransactionCheckRequestId++;
    const {transaction, account, chain} = message.checkTransaction;
    chrome.windows.create({
      url: `index.html/#/transaction?id=${id}&transaction=${transaction}&account=${account}&chain=${chain}&origin=${sender.tab.id}`,
      type: 'popup',
      width: 410,
      height: 320,
      top: 100,
      left: 100,
    });
    transactionCheckRequests.set(id, sendResponse);

    return true;
  }
  if (message.fix) {
    const request = transactionCheckRequests.get(message.fix.id);
    if (request) {
      request({action: 'fix'});
      transactionCheckRequests.delete(message.fix.id);
    }
  }
  if (message.cancel) {
    const request = transactionCheckRequests.get(message.cancel.id);
    if (request) {
      request({action: 'cancel'});
      transactionCheckRequests.delete(message.cancel.id);
    }
  }
  if (message.proceed) {
    const request = transactionCheckRequests.get(message.proceed.id);
    if (request) {
      request({action: 'proceed'});
      transactionCheckRequests.delete(message.proceed.id);
    }
  }
});
