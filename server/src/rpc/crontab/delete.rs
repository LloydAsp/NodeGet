use crate::crontab::delete_crontab_by_name;
use crate::token::get::get_token;
use jsonrpsee::core::RpcResult;
use nodeget_lib::permission::data_structure::{Crontab as CrontabPermission, Permission};
use nodeget_lib::permission::token_auth::TokenOrAuth;
use nodeget_lib::utils::get_local_timestamp_ms;
use serde_json::value::RawValue;

pub async fn delete(token: String, name: String) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let token_or_auth = match TokenOrAuth::from_full_token(&token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        let token_info = get_token(&token_or_auth).await?;

        let now = get_local_timestamp_ms().cast_signed();

        if let Some(from) = token_info.timestamp_from
            && now < from
        {
            return Err((102, "Token is not yet valid".to_string()));
        }

        if let Some(to) = token_info.timestamp_to
            && now > to
        {
            return Err((102, "Token has expired".to_string()));
        }

        let has_crontab_delete_permission = token_info.token_limit.iter().any(|limit| {
            limit
                .permissions
                .iter()
                .any(|perm| matches!(perm, Permission::Crontab(CrontabPermission::Delete)))
        });

        if !has_crontab_delete_permission {
            return Err((
                102,
                "Permission Denied: Insufficient Crontab Delete permission".to_string(),
            ));
        }

        let deleted = delete_crontab_by_name(name)
            .await
            .map_err(|e| (103, e.to_string()))?;

        let json_str = format!("{{\"success\":{}}}", deleted);
        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
