use crate::rpc::RpcHelper;
use jsonrpsee::core::RpcResult;
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use nodeget_lib::metadata;
use serde_json::value::RawValue;
use uuid::Uuid;

mod get;
mod write;

#[rpc(server, namespace = "metadata")]
pub trait Rpc {
    #[method(name = "get")]
    async fn get(&self, token: String, uuid: Uuid) -> RpcResult<Box<RawValue>>;

    #[method(name = "write")]
    async fn write(&self, token: String, metadata: metadata::Metadata) -> RpcResult<Box<RawValue>>;
}

pub struct MetadataRpcImpl;

impl RpcHelper for MetadataRpcImpl {}

#[async_trait]
impl RpcServer for MetadataRpcImpl {
    async fn get(&self, token: String, uuid: Uuid) -> RpcResult<Box<RawValue>> {
        get::get(token, uuid).await
    }

    async fn write(&self, token: String, metadata: metadata::Metadata) -> RpcResult<Box<RawValue>> {
        write::write(token, metadata).await
    }
}
