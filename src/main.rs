use anyhow::Result;
use clap::Parser;
use futures::future::join_all;
use log::{debug, error, info};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Certificate, ClientBuilder, Identity, Url,
};
use serde_json::Value;
use std::time::Instant;
use std::{collections::HashMap, sync::Arc};
use std::{fs, process::exit};
use tokio::sync::Semaphore;

mod cli;
mod config;
mod keystores;

use crate::cli::Cli;
use crate::config::Config;
use crate::keystores::{VaultKey, Web3signerFileKeystore, Web3signerFileRaw};

use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let start = Instant::now();
    info!("Starting vault loader...");

    info!("Parsing command line arguments");
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(error) => {
            error!("{}", error);
            exit(1);
        }
    };
    info!("Command line arguments parsed successfully");

    info!("Parsing configuration");
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error);
            exit(2);
        }
    };
    info!("Configuration parsed successfully");

    info!(
        "Reading public keys from {}",
        &config.vault_pubkeys_json_path.canonicalize()?.display()
    );
    let pubkeys: Vec<String> = match fs::read_to_string(&config.vault_pubkeys_json_path) {
        Ok(pubkeys_file) => match serde_json::from_str(&pubkeys_file) {
            Ok(pubkeys) => pubkeys,
            Err(error) => {
                error!("Failed to parse public keys: {}", error);
                exit(3);
            }
        },
        Err(error) => {
            error!("Failed to parse public keys: {}", error);
            exit(3);
        }
    };
    info!("Public keys read successfully");

    info!(
        "Reading vault token from {}",
        &config.vault_token_path.canonicalize()?.display()
    );
    let vault_token = match fs::read_to_string(&config.vault_token_path) {
        Ok(vault_token) => vault_token,
        Err(error) => {
            error!("Failed to read vault token: {}", error);
            exit(4);
        }
    };
    info!("Vault token read successfully");

    info!("Building vault client");
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-vault-token"),
        HeaderValue::from_str(&vault_token)?,
    );
    info!("Checking TLS configuration");
    let vault_cacert = if let Some(vault_cacert) = &config.vault_cacert {
        if let Ok(vault_cacert) = fs::read(vault_cacert) {
            info!("CA certificate provided, TLS authentication enabled");
            Some(Certificate::from_pem(&vault_cacert)?)
        } else {
            info!("CA certificate not provided, TLS authentication disabled");
            None
        }
    } else {
        None
    };

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

    let vault_client = match if let (Some(vault_client_auth), Some(vault_cacert)) =
        (vault_client_auth, vault_cacert)
    {
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
        Ok(vault_client) => vault_client,
        Err(error) => {
            error!("Failed to create vault client: {}", error);
            exit(5);
        }
    };

    info!("Vault client built successfully");

    let semaphore = Arc::new(Semaphore::new(config.vault_max_concurrent_requests));
    // let mut tasks = HashMap::new();
    let mut tasks = vec![];

    for path in pubkeys {
        info!("Requesting private key for {}", path);
        let vault_client = vault_client.clone();
        let url = Url::parse(&format!(
            "{}/v1/{}/{}/vkey",
            &config.vault_addr, &config.vault_path, path,
        ));
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let task = tokio::spawn(async move {
            let url = url.unwrap();
            // let vault_response = vault_client.get(url).send().await?.json::<Value>().await?;
            let vault_response: VaultKey = serde_json::from_value(
                vault_client.get(url).send().await?.json::<Value>().await?["data"]["data"],
            )?;
            drop(permit);
            Result::<reqwest::Response, reqwest::Error>::Ok(vault_response)
        });
        tasks.push((path, task));
    }

    let responses: Vec<_> = join_all(tasks.into_iter().map(|(path, task)| async move {
        match task.await {
            Ok(vault_response) => Ok((path, vault_response)),
            Err(e) => Err((path, e)),
        }
    }))
    .await;

    for response in responses {
        match response {
            Ok((path, vault_response)) => {
                match vault_response {
                    Ok(vault_response) => {
                        let vault_response = vault_response.json::<Value>().await?;
                        let vkey = vault_response["data"]["data"]["vkey"].as_str();
                        info!("Received private key for {}: {:?}", path, vkey);
                        // process the response for the given path
                    }
                    Err(e) => {
                        error!("Failed to retrieve private key for {}: {:?}", path, e);
                    }
                }
            }
            Err((path, e)) => {
                error!("Failed to retrieve private key for {}: {:?}", path, e);
            }
        }
    }

    // let mut results = HashMap::new();

    // let futures = tasks.into_iter().map(|(path, tasks)| async move {
    //     let result = tasks.await;
    //     (path, result)
    // });

    // let results_vec = join_all(futures).await;

    // for (path, result) in results_vec {
    //     results.insert(path, result);
    // }
    //
    //
    // thread::sleep(Duration::from_secs(200));
    // let tasks = join_all(tasks).await;

    // for (path, result) in results {
    //     match result {
    //         Ok(Ok(vkey)) => {
    //             let vkey = &vkey.json::<Value>().await?["data"]["data"]["vkey"];
    //             match vkey.is_null() {
    //                 true => {
    //                     error!("Failed to receive private key for {}", path);
    //                 }
    //                 false => {
    //                     info!("private key for {} received successfully: {}", path, vkey);
    //                 }
    //             }
    //         }
    //         Ok(Err(error)) => {
    //             error!("Failed to receive private key for {}: {}", path, error)
    //         }
    //         Err(_) => {
    //             error!("Failed to receive private key for {}", path);
    //         }
    //     }
    // }

    // for task in tasks {
    //     println!(
    //         "{:?}",
    //         task.unwrap().unwrap().json::<Value>().await?["data"]["data"]["vkey"]
    //     );
    // }

    let end = Instant::now();
    let elapsed = end - start;
    println!("Elapsed time: {:.2?}", elapsed);

    Ok(())
}
