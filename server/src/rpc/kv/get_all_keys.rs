use crate::kv::get_keys_from_kv;
use jsonrpsee::core::RpcResult;
use log::debug;
use nodeget_lib::error::NodegetError;
use serde_json::value::RawValue;

pub async fn get_all_keys(_token: String, namespace: String) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        debug!("KV RPC: Processing get_all_keys request for namespace '{namespace}'");

        let keys = get_keys_from_kv(namespace).await?;

        let json_str = serde_json::to_string(&keys)
            .map_err(|e| NodegetError::SerializationError(format!("Failed to serialize keys: {e}")))?;

        RawValue::from_string(json_str)
            .map_err(|e| NodegetError::SerializationError(format!("{e}")).into())
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
