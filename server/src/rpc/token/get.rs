use crate::token::get::get_token;
use jsonrpsee::core::RpcResult;
use nodeget_lib::permission::token_auth::TokenOrAuth;
use serde_json::value::RawValue;

pub async fn get(token: String) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let token_or_auth = match TokenOrAuth::from_full_token(&token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        let token_info = get_token(&token_or_auth).await?;

        let json_str = serde_json::to_string(&token_info)
            .map_err(|e| (101, format!("Failed to serialize token info: {e}")))?;

        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
