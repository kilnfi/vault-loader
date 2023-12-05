use super::*;

const PUBKEY: &str = "0x8000025593183bad1730e78b87b6bce428492e3bf9142d2609032daf674596f955d6403481c7d84809905a262c0136e2";

#[test]
fn test_vault_key_new_full() {
    let vault_key_json = serde_json::from_str(
        r#"{
      "password": "password",
      "pbkdf2_key": "eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=",
      "raw_unencrypted_key": "0x800a5c977cb95148f71cd731bbfb44633fc3427975686b458d3670bc61150147",
      "realm": "dashboard",
      "scrypt_key": "eyJjcnlwdG8iOiB7ImtkZiI6IHsiZnVuY3Rpb24iOiAic2NyeXB0IiwgInBhcmFtcyI6IHsiZGtsZW4iOiAzMiwgIm4iOiAyNjIxNDQsICJyIjogOCwgInAiOiAxLCAic2FsdCI6ICJmMTlhYmYxMWM0ODNmMWY2MDgwZGZlNjU4OTkxNDEyZTRhOGM3M2U1OTM4YmMzZWE3NDViYzdkMTJhNmJjZDlhIn0sICJtZXNzYWdlIjogIiJ9LCAiY2hlY2tzdW0iOiB7ImZ1bmN0aW9uIjogInNoYTI1NiIsICJwYXJhbXMiOiB7fSwgIm1lc3NhZ2UiOiAiYzc4Yzg5MjViNTNkYTBlYjcwMDY3ODhmZWEzMmY3NzMwYTM0YzllOTI2NTI2N2UzZmIxMjJiYTQyYTFiNjFlZiJ9LCAiY2lwaGVyIjogeyJmdW5jdGlvbiI6ICJhZXMtMTI4LWN0ciIsICJwYXJhbXMiOiB7Iml2IjogIjJhY2M1MDQ5OTc4YTQyYTAxMjE0ZDFhODdjMjBiNTRkIn0sICJtZXNzYWdlIjogIjUzNGVkOTgwNDkxMWM4MGFkMTUxOTg1NWQ4Mjg3MGMwZDYwZTFmZTViMDE3YzZhZTE2ZDI1ZjY5ZjhmODU2MTMifX0sICJkZXNjcmlwdGlvbiI6ICIiLCAicHVia2V5IjogIjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsICJwYXRoIjogIm0vMTIzODEvMzYwMC8wLzAvMCIsICJ1dWlkIjogIjVkMjA3ZTJjLTQwODItNDUwYy04NTBhLTIwMGMwNDVhYmYwZiIsICJ2ZXJzaW9uIjogNH0=",
      "vkey": "eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0="
    }"#,
    );
    assert!(vault_key_json.is_ok());

    let expected_vault_key = VaultKey {
        pubkey: PUBKEY.to_owned(),
        password: Some("password".to_owned()),
        pbkdf2_key: Some("eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=".to_owned()),
        vkey: Some("eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=".to_owned()),
        realm: Some("dashboard".to_owned()),
        scrypt_key: Some("eyJjcnlwdG8iOiB7ImtkZiI6IHsiZnVuY3Rpb24iOiAic2NyeXB0IiwgInBhcmFtcyI6IHsiZGtsZW4iOiAzMiwgIm4iOiAyNjIxNDQsICJyIjogOCwgInAiOiAxLCAic2FsdCI6ICJmMTlhYmYxMWM0ODNmMWY2MDgwZGZlNjU4OTkxNDEyZTRhOGM3M2U1OTM4YmMzZWE3NDViYzdkMTJhNmJjZDlhIn0sICJtZXNzYWdlIjogIiJ9LCAiY2hlY2tzdW0iOiB7ImZ1bmN0aW9uIjogInNoYTI1NiIsICJwYXJhbXMiOiB7fSwgIm1lc3NhZ2UiOiAiYzc4Yzg5MjViNTNkYTBlYjcwMDY3ODhmZWEzMmY3NzMwYTM0YzllOTI2NTI2N2UzZmIxMjJiYTQyYTFiNjFlZiJ9LCAiY2lwaGVyIjogeyJmdW5jdGlvbiI6ICJhZXMtMTI4LWN0ciIsICJwYXJhbXMiOiB7Iml2IjogIjJhY2M1MDQ5OTc4YTQyYTAxMjE0ZDFhODdjMjBiNTRkIn0sICJtZXNzYWdlIjogIjUzNGVkOTgwNDkxMWM4MGFkMTUxOTg1NWQ4Mjg3MGMwZDYwZTFmZTViMDE3YzZhZTE2ZDI1ZjY5ZjhmODU2MTMifX0sICJkZXNjcmlwdGlvbiI6ICIiLCAicHVia2V5IjogIjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsICJwYXRoIjogIm0vMTIzODEvMzYwMC8wLzAvMCIsICJ1dWlkIjogIjVkMjA3ZTJjLTQwODItNDUwYy04NTBhLTIwMGMwNDVhYmYwZiIsICJ2ZXJzaW9uIjogNH0=".to_owned()),
        raw_unencrypted_key: Some("0x800a5c977cb95148f71cd731bbfb44633fc3427975686b458d3670bc61150147".to_owned()),
        ..Default::default()
    };

    let vault_key_result = VaultKey::new(vault_key_json.unwrap(), PUBKEY);
    assert!(vault_key_result.is_ok());
    assert_eq!(vault_key_result.as_ref().unwrap(), &expected_vault_key);

    let expected_web3signer_yaml_config = r#"type: file-raw
keyType: BLS
privateKey: 0x800a5c977cb95148f71cd731bbfb44633fc3427975686b458d3670bc61150147
"#;

    assert_eq!(
        vault_key_result
            .unwrap()
            .to_config()
            .unwrap()
            .to_yaml()
            .unwrap(),
        expected_web3signer_yaml_config
    );
}

#[test]
fn test_vault_key_new_partial_pbkdf2() {
    let vault_key_json = serde_json::from_str(
        r#"{
      "password": "password",
      "vkey": "eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=",
      "pbkdf2_key": "eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0="
    }"#,
    );
    assert!(vault_key_json.is_ok());

    let expected_vault_key = VaultKey {
        pubkey: PUBKEY.to_owned(),
        password: Some("password".to_owned()),
        vkey: Some("eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=".to_owned()),
        pbkdf2_key: Some("eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=".to_owned()),
        realm: None,
        scrypt_key: None,
        raw_unencrypted_key: None,
    };

    let vault_key_result = VaultKey::new(vault_key_json.unwrap(), PUBKEY);
    assert!(vault_key_result.is_ok());
    assert_eq!(vault_key_result.as_ref().unwrap(), &expected_vault_key);

    let expected_web3signer_yaml_config = format!(
        r#"type: file-keystore
keyType: BLS
keystoreFile: keystore-{0}.json
keystorePasswordFile: keystore-{0}.password
"#,
        PUBKEY
    );

    assert_eq!(
        vault_key_result
            .unwrap()
            .to_config()
            .unwrap()
            .to_yaml()
            .unwrap(),
        expected_web3signer_yaml_config
    );
}

#[test]
fn test_vault_key_new_partial_vkey() {
    let vault_key_json = serde_json::from_str(
        r#"{
      "password": "password",
      "vkey": "eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0="
    }"#,
    );
    assert!(vault_key_json.is_ok());

    let expected_vault_key = VaultKey {
        pubkey: PUBKEY.to_owned(),
        password: Some("password".to_owned()),
        vkey: Some("eyJjcnlwdG8iOnsiY2hlY2tzdW0iOnsiZnVuY3Rpb24iOiJzaGEyNTYiLCJtZXNzYWdlIjoiZTMyNjQ5OWNiODg3Mzg4NGIyZGJkODc3ZWRkOWNkOGZhODVjMjQ5ZWQ5N2YzYTBkN2FkMjQ2MTIxMzNmMzExOCIsInBhcmFtcyI6e319LCJjaXBoZXIiOnsiZnVuY3Rpb24iOiJhZXMtMTI4LWN0ciIsIm1lc3NhZ2UiOiJhNGI0OWJkNGVhOGRmNzU4NTJiNzMwMDI4MDI5MzA4MzU4NGJkN2EyZDhmMjAyMTdjNjU3NmIxODU3NGU1NzY5IiwicGFyYW1zIjp7Iml2IjoiYWZkYzM0M2Y4ODk0ODkyMDkyMWM3NzIxNGFlOGFhZmEifX0sImtkZiI6eyJmdW5jdGlvbiI6InBia2RmMiIsIm1lc3NhZ2UiOiIiLCJwYXJhbXMiOnsiYyI6MjYyMTQ0LCJka2xlbiI6MzIsInByZiI6ImhtYWMtc2hhMjU2Iiwic2FsdCI6IjVlODM2MDlkZmFmOGUxNDc2MDM0M2U5NTNkYjdjMWMxZTQ2ZmNmMTEwNGNlNjlhMDUwOTUwNDU5YzFhNzlmZTIifX19LCJkZXNjcmlwdGlvbiI6IiIsInB1YmtleSI6IjgwMDM0ZTAwMjNkNzE3YWRmYjA0OGViODY3YjZmMmMwMWQwNzlhOTE3YmUwNmFmYjk1NDcxZTNkODJkZjI1ODE4MTAzYjMwMDYxYzZmNTBhNTFkNTk2NTNkOTAyZDBmOCIsInBhdGgiOiJtLzEyMzgxLzM2MDAvMC8wLzAiLCJ1dWlkIjoiNWQyMDdlMmMtNDA4Mi00NTBjLTg1MGEtMjAwYzA0NWFiZjBmIiwidmVyc2lvbiI6NH0=".to_owned()),
        pbkdf2_key: None,
        realm: None,
        scrypt_key: None,
        raw_unencrypted_key: None,
    };

    let vault_key_result = VaultKey::new(vault_key_json.unwrap(), PUBKEY);
    assert!(vault_key_result.is_ok());
    assert_eq!(vault_key_result.as_ref().unwrap(), &expected_vault_key);

    let expected_web3signer_yaml_config = format!(
        r#"type: file-keystore
keyType: BLS
keystoreFile: keystore-{0}.json
keystorePasswordFile: keystore-{0}.password
"#,
        PUBKEY
    );

    assert_eq!(
        vault_key_result
            .unwrap()
            .to_config()
            .unwrap()
            .to_yaml()
            .unwrap(),
        expected_web3signer_yaml_config
    );
}

#[test]
fn test_vault_key_new_invalid() {
    let vault_key_json = serde_json::from_str(
        r#"{
        "wrong": "data"
        }"#,
    );
    assert!(vault_key_json.is_ok());
    let vault_key_result = VaultKey::new(vault_key_json.unwrap(), "0x8000025593183bad1730e78b87b6bce428492e3bf9142d2609032daf674596f955d6403481c7d84809905a262c0136e2");
    assert!(vault_key_result.is_err());
}

#[test]
fn test_vault_key_new_partial_invalid_vkey() {
    let vault_key_json = serde_json::from_str(
        r#"{
        "vkey": "data"
        }"#,
    );
    assert!(vault_key_json.is_ok());
    let vault_key_result = VaultKey::new(vault_key_json.unwrap(), "0x8000025593183bad1730e78b87b6bce428492e3bf9142d2609032daf674596f955d6403481c7d84809905a262c0136e2");
    assert!(vault_key_result.is_err());
}

#[test]
fn test_vault_key_new_partial_invalid_password() {
    let vault_key_json = serde_json::from_str(
        r#"{
        "password": "data"
        }"#,
    );
    assert!(vault_key_json.is_ok());
    let vault_key_result = VaultKey::new(vault_key_json.unwrap(), "0x8000025593183bad1730e78b87b6bce428492e3bf9142d2609032daf674596f955d6403481c7d84809905a262c0136e2");
    assert!(vault_key_result.is_err());
}
