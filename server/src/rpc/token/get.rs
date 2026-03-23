use crate::token::get::{get_token, get_token_by_key_or_username};
use crate::token::super_token::check_super_token;
use jsonrpsee::core::RpcResult;
use nodeget_lib::error::NodegetError;
use nodeget_lib::permission::token_auth::TokenOrAuth;
use serde_json::value::RawValue;

pub async fn get(token: String, supertoken: Option<String>) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let token_info = if let Some(supertoken) = supertoken {
            let supertoken_or_auth = TokenOrAuth::from_full_token(&supertoken).map_err(|e| {
                NodegetError::ParseError(format!("Failed to parse supertoken: {e}"))
            })?;

            let is_super_token = check_super_token(&supertoken_or_auth)
                .await
                .map_err(|e| NodegetError::PermissionDenied(format!("{e}")))?;

            if !is_super_token {
                return Err(NodegetError::PermissionDenied(
                    "Only SuperToken can query by username/token_key in token_get".to_owned(),
                )
                .into());
            }

            match TokenOrAuth::from_full_token(&token) {
                Ok(token_or_auth) => get_token(&token_or_auth).await?,
                Err(_) => get_token_by_key_or_username(&token).await?,
            }
        } else {
            let token_or_auth = TokenOrAuth::from_full_token(&token)
                .map_err(|e| NodegetError::ParseError(format!("Failed to parse token: {e}")))?;
            get_token(&token_or_auth).await?
        };

        let json_str = serde_json::to_string(&token_info).map_err(|e| {
            NodegetError::SerializationError(format!("Failed to serialize token info: {e}"))
        })?;

        RawValue::from_string(json_str)
            .map_err(|e| NodegetError::SerializationError(e.to_string()).into())
    };

    match process_logic.await {
        Ok(result) => Ok(result),
        Err(e) => {
            let nodeget_err = nodeget_lib::error::anyhow_to_nodeget_error(&e);
            Err(jsonrpsee::types::ErrorObject::owned(
                nodeget_err.error_code() as i32,
                format!("{nodeget_err}"),
                None::<()>,
            ))
        }
    }
}
