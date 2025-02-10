use anyhow::Result;
use anyhow::{bail, Context};
use sui_config::{
    sui_config_dir, Config, PersistedConfig, SUI_CLIENT_CONFIG, SUI_KEYSTORE_FILENAME,
};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::sui_client_config::SuiClientConfig;
use sui_sdk::{wallet_context::WalletContext, SuiClient};

pub struct SuiClientWithNetwork {
    pub client: SuiClient,
    pub network: String,
}

impl SuiClientWithNetwork {
    pub async fn new(network: &str) -> Result<Self> {
        let conf = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
        if !conf.exists() {
            bail!("Wallet configuration file does not exist. Please create a wallet first.");
        }
        let client_config: SuiClientConfig = PersistedConfig::read(&conf)?;
        let client = client_config
            .get_active_env()?
            .create_rpc_client(Some(std::time::Duration::from_secs(60)), None)
            .await?;
        Ok(Self {
            client,
            network: network.to_owned(),
        })
    }

    pub async fn with_default_network() -> Result<Self> {
        let conf = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
        if !conf.exists() {
            bail!("Wallet configuration file does not exist. Please create a wallet first.");
        }
        let client_config: SuiClientConfig = PersistedConfig::read(&conf)?;
        let network = client_config
            .active_env
            .as_ref()
            .context("No active environment")?;
        let client = client_config
            .get_active_env()?
            .create_rpc_client(Some(std::time::Duration::from_secs(60)), None)
            .await?;
        Ok(SuiClientWithNetwork {
            client,
            network: network.clone(),
        })
    }
}


/*
pub fn retrieve_wallet() -> Result<WalletContext, anyhow::Error> {
    let wallet_conf = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
    let keystore_path = sui_config_dir()?.join(SUI_KEYSTORE_FILENAME);

    // check if a wallet exists and if not, create a wallet and a sui client config
    if !keystore_path.exists() {
        bail!("Keystore file does not exist. Please create a wallet first.");
    }

    if !wallet_conf.exists() {
        bail!("Wallet configuration file does not exist. Please create a wallet first.");
    }

    let mut keystore = FileBasedKeystore::new(&keystore_path)?;
    let mut client_config: SuiClientConfig = PersistedConfig::read(&wallet_conf)?;

    let active_address = client_config.active_address.context("No active address")?;

    let wallet = WalletContext::new(&wallet_conf, Some(std::time::Duration::from_secs(60)), None)?;

    Ok(wallet)
}
*/
