chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log(message, sender.tab.id);
  if (message.checkTransaction) {
    setTimeout(() => {
      sendResponse(true);
    }, 5000);

    return true;
  }
});
