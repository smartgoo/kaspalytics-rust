use crate::kaspad::dirs::get_app_dir;
use kaspa_consensus_core::network::NetworkId;
use kaspa_consensus_core::network::NetworkType;
use std::{env, path::PathBuf, str::FromStr};
use strum_macros::{Display, EnumString};

#[derive(Clone, Display, EnumString, PartialEq)]
pub enum Env {
    #[strum(serialize = "dev")]
    Dev,

    #[strum(serialize = "uat")]
    Uat,

    #[strum(serialize = "prod")]
    Prod,
}

#[derive(Clone)]
pub struct Config {
    pub env: Env,

    pub network_id: NetworkId,

    pub app_dir: PathBuf,
    pub rpc_url: Option<String>,

    pub db_uri: String,

    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_from: String,
    pub smtp_to: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().unwrap();

        let env = Env::from_str(&env::var("ENV").unwrap()).unwrap();

        let network = NetworkType::from_str(&env::var("NETWORK").unwrap()).unwrap();
        let netsuffix = env::var("NETSUFFIX")
            .ok()
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<u32>().ok());
        let network_id = NetworkId::try_new(network)
            .unwrap_or_else(|_| NetworkId::with_suffix(network, netsuffix.unwrap()));

        let app_dir = env::var("APP_DIR")
            .ok()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| get_app_dir(String::from(".rusty-kaspa")));
        let rpc_url = env::var("RPC_URL").ok();
        println!("{:?}", app_dir);

        let db_uri = env::var("DB_URI").unwrap();

        let smtp_host = env::var("SMTP_HOST").unwrap();
        let smtp_port = env::var("SMTP_PORT").unwrap().parse::<u16>().unwrap();
        let smtp_from = env::var("SMTP_FROM").unwrap();
        let smtp_to = env::var("SMTP_TO").unwrap();

        Config {
            env,
            network_id,
            app_dir,
            rpc_url,
            db_uri,
            smtp_host,
            smtp_port,
            smtp_from,
            smtp_to,
        }
    }
}