use crate::token::generate_token::generate_and_store_token;
use jsonrpsee::core::RpcResult;
use log::debug;
use nodeget_lib::permission::create::TokenCreationRequest;
use nodeget_lib::permission::token_auth::TokenOrAuth;
use serde_json::value::RawValue;

pub async fn create(father_token: String, token_creation: TokenCreationRequest) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let father_token_or_auth = match TokenOrAuth::from_full_token(&father_token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        debug!("Token RPC: Processing token creation request");

        let (key, secret) = generate_and_store_token(
            &father_token_or_auth,
            token_creation.timestamp_from,
            token_creation.timestamp_to,
            token_creation.token_limit,
            token_creation.username,
            token_creation.password,
        )
        .await
        .map_err(|e| (e.0, e.1))?;

        let json_str = format!(
            "{{\"key\":\"{}\",\"secret\":\"{}\"}}",
            key, secret
        );

        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
