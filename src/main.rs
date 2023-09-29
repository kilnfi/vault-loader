use anyhow::{anyhow, Context, Error, Result};
use clap::Parser;
use futures::future::join_all;
use keystores::Web3signerKeyConfig;
use log::{error, info, warn};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Certificate, Client, ClientBuilder, Identity, Url,
};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use std::{fs, path::Path};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

mod cli;
mod config;
mod keystores;

use crate::cli::Cli;
use crate::config::Config;
use crate::keystores::{VaultKey, Web3signerKeyConfigFormat};

fn parse_public_keys(config: &Config) -> Result<Vec<String>> {
    match config.vault_pubkeys_json_path.canonicalize() {
        Ok(pubkeys_json_path) => {
            info!(
                "Reading public keys from file: {}",
                pubkeys_json_path.display()
            );
        }
        Err(error) => {
            error!("Failed to canonicalize public keys file path: {}", error);
            return Err(error).context("Failed to canonicalize public keys file path");
        }
    }

    match fs::read_to_string(&config.vault_pubkeys_json_path) {
        Ok(pubkeys_file) => match serde_json::from_str(&pubkeys_file) {
            Ok(pubkeys) => Ok(pubkeys),
            Err(error) => {
                error!("Failed to parse public keys from file: {}", error);
                Err(error).context("Failed to parse public keys from file")
            }
        },
        Err(error) => {
            error!("Failed to parse public keys: {}", error);
            Err(error).context("Failed to parse public keys from file")
        }
    }
}

fn parse_configuration(args: &Cli) -> Result<Config> {
    match Config::new(args) {
        Ok(config) => Ok(config),
        Err(error) => {
            error!("Failed to parse configuration: {}", error);
            Err(error).context("Failed to parse configuration")
        }
    }
}

fn build_vault_client(config: &Config) -> Result<Client> {
    info!("Reading vault token from file",);
    let vault_token = parse_token(config)?;
    info!("Vault token read successfully");

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-vault-token"),
        HeaderValue::from_str(&vault_token)?,
    );

    info!("Checking TLS configuration");
    let vault_cacert = config.vault_cacert.as_ref().and_then(|vault_cacert| {
        if let Ok(vault_cacert) = fs::read(vault_cacert) {
            info!("CA certificate provided, TLS authentication enabled");
            Some(Certificate::from_pem(&vault_cacert).ok()).flatten()
        } else {
            info!("CA certificate not provided, TLS authentication disabled");
            None
        }
    });

    let vault_client_auth = if let (Some(vault_client_cert), Some(vault_client_key)) =
        (&config.vault_client_cert, &config.vault_client_key)
    {
        if let (Ok(vault_client_cert), Ok(vault_client_key)) = (
            fs::read_to_string(vault_client_cert),
            fs::read_to_string(vault_client_key),
        ) {
            info!("Client certificate and key provided, TLS authentication enabled");
            Some(Identity::from_pem(
                (vault_client_cert + &vault_client_key).as_bytes(),
            )?)
        } else {
            info!("Client certificate and key not provided, TLS authentication disabled");
            None
        }
    } else {
        None
    };

    match if let (Some(vault_client_auth), Some(vault_cacert)) = (vault_client_auth, vault_cacert) {
        info!("Building Vault client with TLS authentication");
        ClientBuilder::new()
            .add_root_certificate(vault_cacert)
            .identity(vault_client_auth)
            .default_headers(headers)
            .use_rustls_tls()
            .build()
    } else {
        info!("Building Vault client without TLS authentication");
        ClientBuilder::new().default_headers(headers).build()
    } {
        Ok(vault_client) => Ok(vault_client),
        Err(error) => {
            error!("Failed to create vault client: {}", error);
            Err(error).context("Failed to create vault client")
        }
    }
}

fn parse_token(config: &Config) -> Result<String> {
    match config.vault_token_path.canonicalize() {
        Ok(vault_token_path) => {
            info!(
                "Reading vault token from file: {}",
                vault_token_path.display()
            );
        }
        Err(error) => {
            error!("Failed to canonicalize vault token file path: {}", error);
            return Err(error).context("Failed to canonicalize vault token file path");
        }
    }

    match fs::read_to_string(&config.vault_token_path) {
        Ok(token) => Ok(token),
        Err(error) => {
            error!("Failed to read vault token: {}", error);
            Err(error).context("Failed to read vault token from file")
        }
    }
}

async fn fetch_vault_key(vault_client: Client, url: Url, pubkey: &str) -> Result<VaultKey, Error> {
    let mut vault_key = VaultKey::new(
        vault_client
            .get(url.clone())
            .send()
            .await?
            .json::<Value>()
            .await?["data"]["data"]
            .clone(),
        pubkey,
    );
    while vault_key.is_err() {
        vault_key = VaultKey::new(
            vault_client
                .get(url.clone())
                .send()
                .await?
                .json::<Value>()
                .await?["data"]["data"]
                .clone(),
            pubkey,
        );
    }
    vault_key
}

async fn write_vault_key(
    web3signer_key_config: &Web3signerKeyConfigFormat,
    path: &Path,
) -> Result<(), Error> {
    match web3signer_key_config {
        Web3signerKeyConfigFormat::Web3signerFileRaw(config) => {
            let path_config = path.join(&config.filename);
            let mut file_config = File::create(path_config).await?;
            file_config.write_all(config.to_yaml()?.as_bytes()).await?;
            Ok(())
        }
        Web3signerKeyConfigFormat::Web3signerFileKeystore(config) => {
            let path_config = path.join(&config.filename);
            let path_keystore = path.join(&config.keystore_file);
            let path_password = path.join(&config.keystore_password_file);
            let mut file_config = File::create(path_config).await?;
            let mut file_keystore = File::create(path_keystore).await?;
            let mut file_password = File::create(path_password).await?;
            file_config.write_all(config.to_yaml()?.as_bytes()).await?;
            file_keystore
                .write_all(serde_json::to_vec(&config.keystore_file_content)?.as_ref())
                .await?;
            file_password
                .write_all(config.keystore_password_file_content.as_bytes())
                .await?;
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let args = Cli::parse();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting vault loader...");

    info!("Parsing configuration");
    let config = parse_configuration(&args)?;
    info!("Configuration parsed successfully");

    info!("Reading public keys from file");
    let pubkeys = parse_public_keys(&config)?;
    info!("Public keys read from file successfully");

    info!("Building vault client");
    let vault_client = build_vault_client(&config)?;
    info!("Vault client built successfully");

    let semaphore = Arc::new(Semaphore::new(config.vault_max_concurrent_requests));
    let mut tasks = vec![];

    // for pubkey in &pubkeys {
    //     info!("Requesting private key for {}", pubkey);
    //     let url = Url::parse(&format!(
    //         "{}/v1/{}/{}/vkey",
    //         &config.vault_addr, &config.vault_path, pubkey,
    //     ))?;
    //     let vault_client_clone = vault_client.clone();
    //     let semaphore_clone = semaphore.clone();
    //     let pubkey_clone = pubkey.clone();

    //     let task = tokio::spawn(async move {
    //         Retry::spawn(retry_strategy.clone(), || async move {
    //             fetch_vault_key(&vault_client_clone, url, pubkey.to_string(), semaphore).await
    //         })
    //         .await
    //     })
    //     .await;

    //     tasks.push((pubkey, task));
    // }

    // for pubkey in &pubkeys {
    //     info!("Requesting private key for {}", pubkey);
    //     let vault_client = vault_client.clone();
    //     let url = Url::parse(&format!(
    //         "{}/v1/{}/{}/vkey",
    //         &config.vault_addr, &config.vault_path, pubkey,
    //     ))?;
    //     let semaphore_clone = semaphore.clone();
    //     let pubkey_clone = pubkey.clone();

    //     // Use tokio-retry to retry the asynchronous operation
    //     let task = Retry::spawn(retry_strategy.clone(), || async move {
    //         let permit = semaphore_clone.acquire_owned().await?;
    //         let vault_response = &vault_client
    //             .get(url.clone())
    //             .send()
    //             .await?
    //             .json::<Value>()
    //             .await?["data"]["data"];
    //         let vault_key = VaultKey::new(vault_response.clone(), pubkey_clone);
    //         drop(permit);
    //         vault_key
    //     })
    //     .await;
    //     tasks.push((pubkey, task));
    // }

    // for pubkey in &pubkeys {
    //     info!("Requesting private key for {}", pubkey);
    //     let vault_client = vault_client.clone();
    //     let url = Url::parse(&format!(
    //         "{}/v1/{}/{}/vkey",
    //         &config.vault_addr, &config.vault_path, pubkey,
    //     ))?;
    //     let pubkey_clone = pubkey.clone();
    //     let permit = semaphore.clone().acquire_owned().await?;
    //     let retry_strategy_clone = retry_strategy.clone();
    //     let task = tokio::spawn(async move {
    //         Retry::spawn(retry_strategy_clone, || async {
    //             let vault_response =
    //                 &vault_client.get(url).send().await?.json::<Value>().await?["data"]["data"];
    //             let vault_key = VaultKey::new(vault_response.clone(), pubkey_clone);
    //             drop(permit);
    //             vault_key
    //         })
    //         .await
    //     });
    //     tasks.push((pubkey, task));
    // }

    for pubkey in &pubkeys {
        info!("Requesting private key for {}", pubkey);
        let vault_client = vault_client.clone();
        let permit = semaphore.clone().acquire_owned().await?;
        let url = Url::parse(&format!(
            "{}/v1/{}/{}/vkey",
            &config.vault_addr, &config.vault_path, pubkey,
        ))?;
        let pubkey_clone = pubkey.clone();
        // let task = fetch_vault_key(
        //     vault_client.clone(),
        //     url,
        //     pubkey.to_string(),
        //     semaphore.clone(),
        // );
        // let task = tokio::spawn(async move {
        //     let vault_response =
        //         &vault_client.get(url).send().await?.json::<Value>().await?["data"]["data"];
        //     let vault_key = VaultKey::new(vault_response.clone(), pubkey_clone);
        //     drop(permit);
        //     vault_key
        // });
        // let task = tokio::spawn(async move {
        //     let vault_key = fetch_vault_key(vault_client.clone(), url, pubkey_clone).await;
        //     drop(permit);
        //     vault_key
        // });
        let task = tokio::spawn(async move {
            let mut vault_key = VaultKey::new(
                vault_client
                    .get(url.clone())
                    .send()
                    .await?
                    .json::<Value>()
                    .await?["data"]["data"]
                    .clone(),
                &pubkey_clone,
            );
            while vault_key.is_err() {
                warn!("Retrying private key for {}", &pubkey_clone);
                vault_key = VaultKey::new(
                    vault_client
                        .get(url.clone())
                        .send()
                        .await?
                        .json::<Value>()
                        .await?["data"]["data"]
                        .clone(),
                    &pubkey_clone,
                );
            }
            drop(permit);
            vault_key
        });
        tasks.push((pubkey, task));
    }

    // let responses: Vec<_> = join_all(tasks.into_iter().map(|(pubkey, task)| async move {
    //     match task.await {
    //         Ok(result) => match result {
    //             Ok(vault_key) => {
    //                 info!("Received private key for: {}", pubkey);
    //                 Ok((pubkey, vault_key))
    //             }
    //             Err(e) => {
    //                 error!("Failed to retrieve private key for {}: {}", pubkey, e);
    //                 Err((pubkey, anyhow!(e)))
    //             }
    //         },
    //         Err(e) => {
    //             error!("Failed to retrieve private key for {}: {}", pubkey, e);
    //             Err((pubkey, anyhow!(e)))
    //         }
    //     }
    // }))
    // .await;

    let responses: Vec<_> = join_all(tasks.into_iter().map(|(pubkey, task)| async move {
        match task.await {
            Ok(result) => match result {
                Ok(vault_key) => {
                    info!("Received private key for: {}", pubkey);
                    Ok((pubkey, vault_key))
                }
                Err(e) => {
                    error!("Failed to retrieve private key for {}: {:?}", pubkey, e);
                    Err((pubkey, anyhow!(e)))
                }
            },
            Err(e) => Err((pubkey, anyhow!(e))),
        }
    }))
    .await;
    // let responses: Vec<_> = join_all(tasks.into_iter().map(|(pubkey, task)| async move {
    //     match task.await {
    //         Ok(vault_key) => {
    //             info!("Received private key for: {}", pubkey);
    //             Ok((pubkey, vault_key))
    //         }
    //         Err(e) => {
    //             error!("Failed to retrieve private key for {}: {}", pubkey, e);
    //             Err((pubkey, anyhow!(e)))
    //         }
    //     }
    // }))
    // .await;

    let semaphore = Arc::new(Semaphore::new(config.max_open_file_descriptors));
    let mut tasks = vec![];

    for response in responses {
        match response {
            Ok((pubkey, vault_key)) => {
                info!("Writing private key for {}", pubkey);
                let permit = semaphore.clone().acquire_owned().await?;
                let web3signer_key_store_path = config.web3signer_key_store_path.clone();
                let task = tokio::spawn(async move {
                    let write =
                        write_vault_key(&vault_key.to_config()?, &web3signer_key_store_path).await;
                    drop(permit);
                    write
                });
                tasks.push((pubkey, task));
            }
            Err((pubkey, e)) => {
                error!("Failed to write private key for {}: {}", pubkey, e);
            }
        }
    }

    let _writes: Vec<_> = join_all(tasks.into_iter().map(|(pubkey, task)| async move {
        match task.await {
            Ok(result) => match result {
                Ok(_) => {
                    info!("Private key written successfully for: {}", pubkey);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to write private key for {}: {}", pubkey, e);
                    Err(anyhow!(e))
                }
            },
            Err(e) => {
                error!("Failed to write private key for {}: {}", pubkey, e);
                Err(anyhow!(e))
            }
        }
    }))
    .await;

    let end = Instant::now();
    let elapsed = end - start;
    println!("Elapsed time: {:.2?}", elapsed);

    Ok(())
}
