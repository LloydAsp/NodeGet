use std::{result::Result as StdResult};
use rquickjs::Error;
use serde_json::value::RawValue;
use tokio::sync::mpsc::Receiver;
use crate::js_runtime::js_error;
use crate::rpc::get_modules;

pub async fn js_nodeget(json: String) -> StdResult<String, Error> {
    let rpc_module = get_modules();

    let (resp, _stream) = match rpc_module.raw_json_request(&json, 16).await {
        Ok(resp) => resp,
        Err(e) => {
            return Err(js_error("jsonrpc_module", e.to_string()));
        }
    };

    Ok(resp.to_string())
}