use crate::token::generate_token::generate_and_store_token;
use crate::token::get::get_token;
use crate::token::split_username_password;
use jsonrpsee::proc_macros::rpc;
use log::debug;
use migration::async_trait::async_trait;
use nodeget_lib::monitoring::data_structure::{DynamicMonitoringData, StaticMonitoringData};
use nodeget_lib::permission::create::TokenCreationRequest;
use nodeget_lib::utils::error_message::generate_error_message;
use serde_json::{Value, json};

#[rpc(server, namespace = "token")]
pub trait Rpc {
    #[method(name = "get")]
    async fn get(&self, token: String) -> Value;

    #[method(name = "create")]
    async fn create(&self, token_creation: TokenCreationRequest) -> Value;
}
pub struct TokenRpcImpl;

#[async_trait]
impl RpcServer for TokenRpcImpl {
    async fn get(&self, token: String) -> Value {
        let (token_arg, username_arg, password_arg) =
            if let Ok((u, p)) = split_username_password(&token) {
                debug!("Token RPC: Detected Username|Password login");
                (None, Some(u.to_string()), Some(p.to_string()))
            } else {
                debug!("Token RPC: Detected Token string login");
                (Some(token), None, None)
            };

        match get_token(token_arg, username_arg, password_arg).await {
            Ok(token_info) => serde_json::to_value(token_info).unwrap_or_else(|e| {
                generate_error_message(101, &format!("Failed to serialize token info: {e}"))
            }),
            Err((code, msg)) => generate_error_message(code, &msg),
        }
    }

    async fn create(&self, token_creation: TokenCreationRequest) -> Value {
        let (super_token_arg, super_username_arg, super_password_arg) =
            if let Ok((u, p)) = split_username_password(&token_creation.father_token) {
                debug!("Token RPC: Detected Username|Password login");
                (None, Some(u.to_string()), Some(p.to_string()))
            } else {
                debug!("Token RPC: Detected Token string login");
                (Some(token_creation.father_token), None, None)
            };

        let (key, secret) = match generate_and_store_token(
            super_token_arg,
            super_username_arg,
            super_password_arg,
            token_creation.timestamp_from,
            token_creation.timestamp_to,
            token_creation.token_limit,
            token_creation.username,
            token_creation.password,
        )
        .await
        {
            Ok((key, secret)) => (key, secret),
            Err(e) => {
                return generate_error_message(e.0, &e.1);
            }
        };

        json!({
            "key": key,
            "secret": secret,
        })
    }
}
