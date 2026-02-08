use crate::token;
use crate::token::get::get_token;
use crate::token::super_token::check_super_token;
use jsonrpsee::core::RpcResult;
use nodeget_lib::permission::token_auth::TokenOrAuth;
use serde_json::value::RawValue;

pub async fn delete(token: String, target_token_key: Option<String>) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let token_or_auth = match TokenOrAuth::from_full_token(&token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        let current_token_info = get_token(&token_or_auth).await?;

        let is_super_token = check_super_token(&token_or_auth)
            .await
            .map_err(|e| (102, e))?;

        let json_str = if is_super_token {
            let Some(target_key_to_delete) = target_token_key else {
                return Err((
                    102,
                    "Target token key is required for SuperToken deletion".to_string(),
                ));
            };

            let delete_result = token::delete_token_by_key(target_key_to_delete.clone())
                .await
                .map_err(|e| (103, e.to_string()))?;

            if delete_result.rows_affected > 0 {
                format!(
                    "{{\"success\":true,\"message\":\"Token {} deleted successfully by SuperToken\",\"rows_affected\":{}}}",
                    target_key_to_delete, delete_result.rows_affected
                )
            } else {
                format!(
                    "{{\"success\":false,\"message\":\"Token {} not found\"}}",
                    target_key_to_delete
                )
            }
        } else {
            if target_token_key.is_some() {
                return Err((
                    102,
                    "Insufficient permission to delete other tokens".to_string(),
                ));
            }

            let target_key_to_delete = current_token_info.token_key.clone();

            let delete_result = token::delete_token_by_key(target_key_to_delete.clone())
                .await
                .map_err(|e| (103, e.to_string()))?;

            if delete_result.rows_affected > 0 {
                format!(
                    "{{\"success\":true,\"message\":\"Own token deleted successfully\",\"rows_affected\":{}}}",
                    delete_result.rows_affected
                )
            } else {
                "{\"success\":false,\"message\":\"Own token not found\"}".to_string()
            }
        };

        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
