import { A } from "@solidjs/router";

export default function Home() {
  return (
    <>
      <h1>Dominion protector</h1>
      <article class="card">
        <p>
          An <strong>AI-powered security suite</strong> for the{' '}
          <a href="https://sui.io/" target="_blank">
            <strong>Sui blockchain</strong>
          </a>
          , protecting users from <strong>malicious smart contracts</strong> and{' '}
          <strong>risky transactions</strong>.
        </p>
      </article>
      <h1>Components</h1>
      <section>
        <A href="/wallet-protector">
          <article class="card">
            <h2>Wallet protector</h2>
            <p>
              The Wallet Protector Chrome extension installs a{' '}
              <strong>secured version</strong> of every wallet in your Chrome
              browser, intercepting transactions for{' '}
              <strong>security checks</strong> before signing. It analyzes risks
              using <strong>AI and formal verification</strong>, alerts users to
              potential threats, and offers options to{' '}
              <strong>enhance security or reject unsafe transactions</strong>.
              Signing is delegated to the original wallet, ensuring{' '}
              <strong>private keys remain secure</strong> while adding a
              critical layer of protection against malicious contracts and
              scams.
            </p>
          </article>
        </A>
        <A href="/explorer">
          <article class="card">
            <h2>AI blockchain explorer</h2>
            <p>
              The <strong>AI-enhanced blockchain explorer</strong> provides{' '}
              <strong>clear, user-friendly explanations</strong> of every smart
              contract function and transaction on the Sui blockchain. It
              translates complex contract logic into{' '}
              <strong>easy-to-understand insights</strong>, helping users assess
              risks and make informed decisions.
            </p>
            <p>
              It also features a <strong>smart transaction builder</strong>,
              allowing users to{' '}
              <strong>
                create secure transactions without interacting with third-party
                apps
              </strong>
              , reducing the risk of phishing or compromised interfaces. Simply
              describe the action — e.g.,
              <strong>
                "Swap 10 USDC for SUI, stake it, and use it as collateral in a
                lending pool"
              </strong>
              — and the system will generate a safe, optimized transaction for
              execution.
            </p>
          </article>
        </A>
      </section>
    </>
  );
}
