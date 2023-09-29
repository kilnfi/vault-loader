use super::*;

#[test]
fn test_config_ok() {
    let args = Cli {
        config: Some(PathBuf::from("/etc/vault_loader/config.yaml")),
        vault_cacert: Some(PathBuf::from("/vault_loader/ca.pem")),
        vault_client_cert: Some(PathBuf::from("/vault_loader/client.pem")),
        vault_client_key: Some(PathBuf::from("/vault_loadder/client.key")),
        vault_path: Some("ethereum/keys".to_string()),
        vault_addr: Some("https://vault.domain.name".to_string()),
        vault_token_path: Some(PathBuf::from("vault_loader/token")),
        vault_pubkeys_json_glob: Some("/vault_loader/pubkeys.json".to_string()),
        vault_max_concurrent_requests: None,
        web3signer_key_store_path: Some(PathBuf::from("/web3signer")),
        max_open_file_descriptors: Some(1024),
    };
    let config = Config::new(&args);
    assert!(config.is_ok());
    assert_eq!(config.unwrap().vault_max_concurrent_requests, 20);
}

#[test]
fn test_config_err() {
    let args = Cli {
        config: Some(PathBuf::from("/etc/vault_loader/config.yaml")),
        vault_cacert: Some(PathBuf::from("/vault_loader/ca.pem")),
        vault_client_cert: Some(PathBuf::from("/vault_loader/client.pem")),
        vault_client_key: None,
        vault_path: Some("ethereum/keys".to_string()),
        vault_addr: Some("https://vault.domain.name".to_string()),
        vault_token_path: Some(PathBuf::from("vault_loader/token")),
        vault_pubkeys_json_glob: Some("/vault_loader/pubkeys.json".to_string()),
        vault_max_concurrent_requests: None,
        web3signer_key_store_path: Some(PathBuf::from("/web3signer")),
        max_open_file_descriptors: None,
    };
    let config = Config::new(&args);
    assert!(config.is_err());
}
