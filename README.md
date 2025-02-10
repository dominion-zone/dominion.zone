Here's the **finalized README.md** with the correct installation steps for **Revela**, ensuring it is cloned and built separately before installing.  

---

# **Dominion Protector**  

**AI-powered security suite for the Sui blockchain**  

## **Overview**  
Dominion Protector is an **AI-powered security suite** designed to **detect and prevent malicious smart contracts and risky transactions** on the **Sui blockchain**. It includes:  

- **Wallet Protector Chrome Extension** – Intercepts transactions before signing, warns users, and provides safer alternatives.  
- **AI-Powered Blockchain Explorer** – Translates contract behavior into human-readable explanations.  
- **Scam Simulation** – A test environment to demonstrate how scams work and how Dominion Protector prevents them.  

## **Features**  
✅ **Real-time Transaction Analysis** – Detects and warns about scams before signing.  
✅ **AI & Formal Verification** – Ensures transaction safety using AI-powered risk assessment.  
✅ **Human-Readable Explanations** – Explains smart contract behavior in simple terms.  
✅ **Fix & Protect Transactions** – Offers safe alternatives to run transactions securely.  
✅ **Chrome Extension Integration** – Works with existing wallets without exposing private keys.  

---

## **Project Structure**  

```
dominion.zone/
├── rs/dominion-protector    # Rust-based CLI/REST service for blockchain analysis & AI results
├── ts/                      # TypeScript-based services for scam test, frontend, and extension
│   ├── app/                 # Dominion.zone frontend
│   ├── dominion-protector-chrome/  # Wallet Protector Chrome extension
│   ├── scamtest-app/        # scamtest.xyz frontend (Scam simulation)
│   ├── scamtest-cli/        # CLI/REST for scam test service
│   ├── scamtest-sdk/        # SDK for interacting with scam test contract
│   ├── solid-sui/           # SUI blockchain utilities for Solid.js
├── sui/scamtest             # Scam simulation smart contract
├── Cargo.toml               # Rust workspace configuration
├── package.json             # TypeScript root
├── LICENSE                  # Project license
├── pnpm-workspace.yaml      # pnpm workspace configuration
```

---

## **How the Rust Analysis Service Works**  

The **Rust-based backend** is responsible for **decompiling smart contracts, analyzing them using an LLM (via Atoma Network), storing the results in PostgreSQL, and serving them via a REST API**.

### **Step-by-Step Breakdown**  

1. **Decompiling Smart Contract Modules**  
   - The service uses [Revela](https://github.com/verichains/revela) to **decompile contract modules into readable Rust code**.  
   - Each **module is processed separately** to extract key logic.  

2. **Feeding Each Module to LLM for Analysis**  
   - The decompiled contract modules are **sent to Atoma Network's LLM**.  
   - The service asks **structured questions** about:
     - The contract’s **overall purpose**
     - Potential **security vulnerabilities**
     - **Hidden logic** that could lead to scams  

3. **Collecting AI Responses in PostgreSQL**  
   - Each module’s analysis is **stored in a PostgreSQL database**.  
   - The answers include:
     - **General contract behavior**
     - **Security classification** (e.g., Critical Risk, High Risk, etc.)
     - **Specific vulnerabilities detected**  

4. **Serving Data via REST API**  
   - The Rust service runs as an **HTTP REST API** when executed with:  
     ```sh
     cargo run -- serve
     ```
   - API clients (like the frontend or Chrome extension) **query contract details** before executing a transaction.  

5. **On-Demand Contract Exploration (Slow Process)**  
   - The API can be used to **explore new contracts dynamically**.  
   - However, AI analysis **take time**, so it is recommended to **preprocess contracts before querying**.  

---

## **How ScamTest Works**  

ScamTest is a **deceptive smart contract simulation** designed to show users how scams operate and how **Wallet Protector** prevents them.  

### **Step-by-Step Breakdown**  

1. **User clicks "Try" on scamtest.xyz**  
   - This sends a request to the **ScamTest CLI service** (`scamtest-cli`).  

2. **ScamTest CLI triggers a blockchain transaction**  
   - The contract **creates a new slot** for the user, which is valid for **a few seconds**.  
   - The slot is stored as a **SHA256 hash** of a random number.  

3. **User receives the original number**  
   - This number acts as the **key** to claim the slot.  
   - The frontend **simulates the claim transaction** and shows **win tokens incoming**.  

4. **User approves and submits the actual transaction**  
   - However, by the time the transaction executes, the **slot expires**.  

5. **Result: No win tokens granted**  
   - The user checks the blockchain explorer and sees **nothing was received**.  
   - The contract appeared fair **in the simulation**, but in reality, it **tricked the user** into believing they would receive tokens.  

---

## **Installation & Setup**  

### **1. Clone the repository**  
```sh
git clone https://github.com/dominion-zone/dominion.zone.git
cd dominion.zone
```

### **2. Install Dependencies**  

#### **Rust Components (Blockchain Analysis & AI Service)**  
**Install Rust & Cargo** (if not installed):  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### **Install Revela for Contract Decompilation**  
[Revela](https://github.com/verichains/revela) is required to decompile smart contracts.  
Clone and build Revela separately:  
```sh
git clone https://github.com/verichains/revela.git
cd revela
cargo install --path third_party/move/tools/revela
cd ..
```

#### **Set up PostgreSQL for AI result buffering**  
Install PostgreSQL:  
```sh
sudo apt update && sudo apt install postgresql postgresql-contrib
```
Start and configure the database:  
```sh
sudo systemctl start postgresql
sudo -u postgres psql
```
Run the following inside PostgreSQL:  
```sql
CREATE DATABASE dominion_protector;
CREATE USER dominion_user WITH ENCRYPTED PASSWORD 'securepassword';
GRANT ALL PRIVILEGES ON DATABASE dominion_protector TO dominion_user;

-- Create ENUM types
CREATE TYPE owner_type AS ENUM ('AddressOwner', 'ObjectOwner', 'Shared', 'Immutable', 'ConsensusV2');
CREATE TYPE security_level AS ENUM (
    'Critical Risk',
    'High Risk',
    'Medium Risk',
    'Low Risk',
    'Best Practices Compliant',
    'Unknown / Unassessed'
);
CREATE TYPE entity_kind AS ENUM ('parameter', 'created');
```
Exit PostgreSQL with `\q`.  

Set up environment variables:  
```sh
export DATABASE_URL="postgres://dominion_user:securepassword@localhost/dominion_protector"
```

#### **Set up Atoma Network LLM Provider**  
To use AI-powered contract analysis, set the **AI_API_KEY** for **Atoma Network**:  
```sh
export AI_API_KEY="your-atoma-api-key"
```

#### **TypeScript Components (Frontend, Scamtest, Chrome Extension)**  
Install dependencies using pnpm:  
```sh
pnpm install
```

---

## **Running the Services**  

### **1. Start the Rust Analysis Service**  
```sh
cd rs/dominion-protector
cargo run serve 0.0.0.0:7000
```

### **2. Start the TypeScript Services**  

- **Frontend:**  
  ```sh
  cd ts/app
  pnpm dev
  ```

- **Chrome Extension Development:**  
  ```sh
  cd ts/dominion-protector-chrome
  pnpm build:dominion
  pnpm dev
  ```

- **Scamtest Service:**  
  ```sh
  cd ts/scamtest-cli
  pnpm cli serve --port 8237
  ```

- **Scamtest.xyz Frontend:**  
  ```sh
  cd ts/scamtest-app
  pnpm dev
  ```

### **3. Start PostgreSQL** (if not already running)  
```sh
sudo systemctl start postgresql
```

---

## **Installing the Chrome Extension Locally**  

To install and test the **Wallet Protector Chrome Extension**, follow these steps:  

1. **Enable Developer Mode in Chrome**:  
   - Open Chrome and go to `chrome://extensions/`.  
   - Toggle **"Developer mode"** in the top-right corner.  

2. **Load the unpacked extension**:  
   - Click **"Load unpacked"**.  
   - Select the `dist/` folder inside `ts/dominion-protector-chrome`.  
   - The extension should now be installed and active.  

---

## **Testing the Scam Simulation**  

1. Open [**dominion.zone**](https://dominion.zone)  
2. Click **"Test Your Wallet"** to visit the scam test site  
3. Connect a wallet & simulate a scam  
4. Enable **Wallet Protector** & rerun the test  
5. See how Dominion Protector **detects and blocks the scam**  

---

## **Contributing**  
We welcome contributions! To contribute:  
1. Fork the repository  
2. Create a new branch (`feature/your-feature`)  
3. Commit your changes  
4. Open a pull request  

---

## **License**  
This project is licensed under the **BSD 2-Clause License** – see the [LICENSE](LICENSE) file for details.  

---

## **Stay Updated**  
🔹 **Website:** [dominion.zone](https://dominion.zone)   

🚀 **Protect your assets. Stay ahead of scams.**