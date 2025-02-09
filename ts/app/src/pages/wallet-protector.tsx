export default function WalletProtector() {
  return (
    <div>
      <h1>Wallet Protector: AI-Powered Security for Your Web3 Transactions</h1>

      <section class="card">
        <p>
          The <strong>Wallet Protector Chrome Extension</strong> safeguards your
          Web3 transactions by intercepting and analyzing them before signing.
          It detects potential security threats such as phishing attacks,
          malicious smart contracts, and unauthorized approvals.
        </p>
      </section>

      <h1>How It Works</h1>
      <section class="card">
        <ul>
          <li>
            <strong>Wraps Every Installed Wallet:</strong> Installs a protected
            version of every Web3 wallet in your browser alongside the original
            version, with the protected version named starting with "DP:" to
            differentiate it.
            <div style="width: 100%; text-align: center;">
              <img
                src="./addWallets.png"
                alt="Wallet Protector Installation Screenshot"
                class="screenshot"
              />
            </div>
          </li>
          <li>
            <strong>Analyzes Transactions:</strong> Uses AI to detect risks,
            explain security threats, and provide user-friendly alerts.
          </li>
          <li>
            <strong>Security Enhancements:</strong> Offers a “Fix”, “Cancel” and
            “Proceed” buttons to improve safety before signing.
            <div style="width: 100%; text-align: center;">
              <img
                src="./warning.png"
                alt="Wallet Protector Warning Screenshot"
                class="screenshot"
              />
            </div>
          </li>
          <li>
            <strong>Delegated Signing:</strong> Transactions are signed by your
            original wallet, ensuring private keys remain secure.
          </li>
        </ul>
      </section>

      <h1>Why Use Wallet Protector?</h1>
      <section class="card">
        <ul>
          <li>✅ AI-powered risk analysis to prevent scams</li>
          <li>✅ Clear, user-friendly transaction explanations</li>
          <li>✅ Non-custodial – your private keys stay secure</li>
          <li>✅ Enhanced security for Web3 interactions</li>
        </ul>
      </section>

      <h1>🚀 Try Wallet Protector (Experimental Version)</h1>
      <section class="card">
        <p>
          <span class="warning">⚠️ Note:</span> This is an{' '}
          <strong>experimental release</strong> for demonstration purposes only.
          Do not use it for real transactions. To install, users must first
          enable development mode in their browser’s extension settings and load
          the extension from a locally unpacked archive.
        </p>

        <a href="./dominion-protector.zip" target="_self" class="install-btn">
          Install Wallet Protector (Demo)
        </a>
      </section>

      <h1>Join the Future of Secure Web3 Transactions</h1>
      <section class="card">
        <p>
          With blockchain security risks on the rise, Wallet Protector provides
          real-time analysis and transparency, ensuring a safer Web3 experience.
          Get involved and help shape the future of secure crypto transactions!
        </p>
      </section>
    </div>
  );
}
