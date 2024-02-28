use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;

#[skip_serializing_none]
#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(name = "vault-toolbox", author, version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    #[serde(skip_serializing)]
    command: Command,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(long, value_name = "PATH")]
    pub vault_cacert: Option<PathBuf>,

    /// Path on the local disk to a single PEM-encoded CA certificate to use
    /// for TLS authentication to the Vault server. If this flag is specified,
    /// -client-key is also required. This can also be specified via the
    /// VAULT_CLIENT_CERT environment variable.
    #[arg(long, value_name = "PATH")]
    pub vault_client_cert: Option<PathBuf>,

    /// Path on the local disk to a single PEM-encoded private key matching the
    /// client certificate from -client-cert. This can also be specified via the
    /// VAULT_CLIENT_KEY environment variable.
    #[arg(long, value_name = "PATH")]
    pub vault_client_key: Option<PathBuf>,

    /// Vault server URL
    #[arg(long, value_name = "URL")]
    pub vault_addr: Option<String>,

    /// Path on the local disk to a file containing the Vault token to use
    #[arg(long, value_name = "PATH")]
    pub vault_token_path: Option<PathBuf>,

    /// Maximum number of concurrent requests to Vault
    #[arg(long, value_name = "PATH")]
    pub vault_max_concurrent_requests: Option<usize>,

    /// Maximum number of concurrent requests to Vault
    #[arg(long, value_name = "FD")]
    pub max_open_file_descriptors: Option<usize>,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
enum Command {
    #[serde(rename = "download")]
    Download {
        /// Path on the local disk to a directory containing the web3signer key store
        #[arg(long, value_name = "PATH")]
        web3signer_key_store_path: Option<PathBuf>,

        /// Path on the K/V store to the secrets to be loaded
        #[arg(long, value_name = "KV_PATH")]
        vault_path: Option<String>,

        /// Path on the local disk to a json file containing a list of public keys to load
        #[arg(long, value_name = "PATH")]
        vault_pubkeys_json_glob: Option<String>,
    },
}
