use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_yaml;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Deserialize, Debug)]
pub struct VaultKey {
    pub vkey: String,
    pub password: String,
    pub pbkdf2_key: Option<String>,
    pub scrypt_key: Option<String>,
    pub raw_unencrypted_key: Option<String>,
    pub realm: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Web3signerKeyConfigFormat {
    FileKeystore(Web3signerFileKeystore),
    FileRaw(Web3signerFileRaw),
}

impl Web3signerKeyConfigFormat {
    pub async fn write_to_yaml_file(&self, path: &Path) -> Result<(), Error> {
        let mut file = File::create(path).await?;
        file.write_all(serde_yaml::to_string(&self)?.as_bytes())
            .await?;
        Ok(())
    }
}

impl VaultKey {
    pub async fn new(object: Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(object["data"]["data"].clone())
    }

    pub async fn parse(&self) -> Result<Web3signerKeyConfigFormat, Error> {
        let key_type = self.vkey.split('-').collect::<Vec<&str>>()[0];
        let key_config = match key_type {
            "file" => Web3signerKeyConfigFormat::FileRaw(Web3signerFileRaw {
                r#type: "file-raw".to_string(),
                key_type: key_type.to_string(),
                private_key_file: format!("{}.key", self.vkey),
            }),
            "file-keystore" => Web3signerKeyConfigFormat::FileKeystore(Web3signerFileKeystore {
                r#type: "file-keystore".to_string(),
                key_type: key_type.to_string(),
                keystore_file: format!("{}.json", self.vkey),
                keystore_password_file: format!("{}.password", self.vkey),
            }),
            _ => return Err(anyhow::anyhow!("Unsupported key type: {}", self.vkey)),
        };
        Ok(key_config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Web3signerFileKeystore {
    pub r#type: String,
    pub key_type: String,
    pub keystore_file: String,
    pub keystore_password_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Web3signerFileRaw {
    pub r#type: String,
    pub key_type: String,
    pub private_key_file: String,
}
