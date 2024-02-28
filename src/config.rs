use crate::cli::Cli;
use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use figment::{
    providers::{Env, Format, Serialized, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(test)]
#[path = "./config_tests.rs"]
mod config_tests;

#[enum_dispatch]
pub trait Configuration {}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub vault_cacert: Option<PathBuf>,
    pub vault_client_cert: Option<PathBuf>,
    pub vault_client_key: Option<PathBuf>,
    pub vault_addr: String,
    pub vault_token_path: PathBuf,
    #[serde(default = "default_vault_max_concurrent_requests")]
    pub vault_max_concurrent_requests: usize,
    #[serde(default = "default_max_open_file_descriptors")]
    pub max_open_file_descriptors: usize,
    pub command: CommandConfig,
}

#[enum_dispatch(Configuration)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CommandConfig {
    #[serde(alias = "upload")]
    ConfigUpload,
    #[serde(alias = "download")]
    ConfigDownload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConfigUpload {
    pub vault_privkeys_json_glob: String,
    pub vault_path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConfigDownload {
    pub vault_pubkeys_json_glob: String,
    pub web3signer_key_store_path: PathBuf,
    pub vault_path: String,
}

fn default_vault_max_concurrent_requests() -> usize {
    20
}

fn default_max_open_file_descriptors() -> usize {
    1024
}

impl Config {
    pub fn new(args: &Cli) -> Result<Self> {
        let mut config = Figment::new();
        if let Some(config_file) = &args.config {
            config = config.merge(Yaml::file(config_file));
        }

        let config = config
            .merge(Env::raw().filter(|env| env.starts_with("VAULT_")))
            .merge(Serialized::defaults(args));

        let has_vault_cacert = config.extract_inner::<PathBuf>("vault_cacert").is_ok();
        let has_vault_client_cert = config.extract_inner::<PathBuf>("vault_client_cert").is_ok();
        let has_vault_client_key = config.extract_inner::<PathBuf>("vault_client_key").is_ok();
        if has_vault_cacert != has_vault_client_cert || has_vault_cacert != has_vault_client_key {
            return Err(anyhow!("vault_cacert, vault_client_cert, and vault_client_key must be set together or not at all"));
        }

        println!("config: {:?}", config.extract_inner::<Config>("")?);
        Ok(config.extract()?)
    }
}
