use anyhow::{anyhow, Error, Result};
use base64::{engine::general_purpose, Engine as _};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(test)]
#[path = "./keystores_tests.rs"]
mod keystores_tests;

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
pub struct VaultKey {
    #[serde(skip_deserializing)]
    pub pubkey: String,
    pub vkey: String,
    pub password: String,
    pub pbkdf2_key: Option<String>,
    pub scrypt_key: Option<String>,
    pub raw_unencrypted_key: Option<String>,
    pub realm: Option<String>,
}

impl VaultKey {
    pub fn new(object: Value, pubkey: String) -> Result<Self, anyhow::Error> {
        let mut vault_key: Self = serde_json::from_value(object.clone())?;
        vault_key.pubkey = pubkey;
        Ok(vault_key)
    }

    pub fn to_config(&self) -> Result<Web3signerKeyConfigFormat, Error> {
        match &self.raw_unencrypted_key {
            Some(raw_unencrypted_key) => Ok(Web3signerKeyConfigFormat::from(Web3signerFileRaw {
                pubkey: self.pubkey.to_string(),
                filename: format!("keystore-{}.yaml", self.pubkey),
                private_key: raw_unencrypted_key.to_string(),
                ..Default::default()
            })),
            None => {
                if let Some(pbkdf2_key) = &self.pbkdf2_key {
                    Ok(Web3signerKeyConfigFormat::from(Web3signerFileKeystore {
                        pubkey: self.pubkey.to_string(),
                        filename: format!("keystore-{}.yaml", self.pubkey),
                        keystore_file: format!("keystore-{}.json", self.pubkey),
                        keystore_file_content: serde_json::from_str(
                            base64_decode(pbkdf2_key)?.as_str(),
                        )?,
                        keystore_password_file: format!("keystore-{}.password", self.pubkey),
                        keystore_password_file_content: self.password.to_string(),
                        ..Default::default()
                    }))
                } else {
                    Ok(Web3signerKeyConfigFormat::from(Web3signerFileKeystore {
                        pubkey: self.pubkey.to_string(),
                        filename: format!("keystore-{}.yaml", self.pubkey),
                        keystore_file: format!("keystore-{}.json", self.pubkey),
                        keystore_file_content: serde_json::from_str(
                            base64_decode(self.vkey.as_str())?.as_str(),
                        )?,
                        keystore_password_file: format!("keystore-{}.password", self.pubkey),
                        keystore_password_file_content: self.password.to_string(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}

#[enum_dispatch]
pub trait Web3signerKeyConfig {
    fn to_yaml(&self) -> Result<String, Error>;
}

#[enum_dispatch(Web3signerKeyConfig)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Web3signerKeyConfigFormat {
    Web3signerFileKeystore,
    Web3signerFileRaw,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Web3signerFileKeystore {
    #[serde(skip_serializing)]
    pub pubkey: String,
    #[serde(skip_serializing)]
    pub filename: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default, rename = "keyType")]
    pub key_type: String,
    #[serde(rename = "keystoreFile")]
    pub keystore_file: String,
    #[serde(skip_serializing)]
    pub keystore_file_content: Value,
    #[serde(rename = "keystorePasswordFile")]
    pub keystore_password_file: String,
    #[serde(skip_serializing)]
    pub keystore_password_file_content: String,
}

impl Default for Web3signerFileKeystore {
    fn default() -> Self {
        Web3signerFileKeystore {
            pubkey: Default::default(),
            filename: "keystore-pubkey.json".to_string(),
            r#type: "file-keystore".to_string(),
            key_type: "BLS".to_string(),
            keystore_file: Default::default(),
            keystore_file_content: Default::default(),
            keystore_password_file: Default::default(),
            keystore_password_file_content: Default::default(),
        }
    }
}

impl Web3signerKeyConfig for Web3signerFileKeystore {
    fn to_yaml(&self) -> Result<String, Error> {
        serde_yaml::to_string(self).map_err(|e| anyhow!(e))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Web3signerFileRaw {
    #[serde(skip_serializing)]
    pub pubkey: String,
    #[serde(skip_serializing)]
    pub filename: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default, rename = "keyType")]
    pub key_type: String,
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

impl Default for Web3signerFileRaw {
    fn default() -> Self {
        Web3signerFileRaw {
            pubkey: Default::default(),
            filename: "keystore-pubkey.json".to_string(),
            r#type: "file-raw".to_string(),
            key_type: "BLS".to_string(),
            private_key: Default::default(),
        }
    }
}

impl Web3signerKeyConfig for Web3signerFileRaw {
    fn to_yaml(&self) -> Result<String, Error> {
        serde_yaml::to_string(self).map_err(|e| anyhow!(e))
    }
}

fn base64_decode(input: &str) -> Result<String, Error> {
    let bytes = general_purpose::STANDARD.decode(input.as_bytes())?;
    let decoded = std::str::from_utf8(&bytes)?.to_string();
    Ok(decoded)
}
