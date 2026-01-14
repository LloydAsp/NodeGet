use crate::entity::static_monitoring;
use crate::DB;
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use nodeget_lib::monitoring::data_structure::StaticMonitoringData;
use nodeget_lib::utils::error_message::generate_error_message;
use sea_orm::{EntityTrait, Set};

#[rpc(server, namespace = "agent")]
pub trait Rpc {
    #[method(name = "report_static")]
    async fn report_static(&self, token: String, data: serde_json::Value) -> serde_json::Value;
}

pub struct AgentRpcImpl;

#[async_trait]
impl RpcServer for AgentRpcImpl {
    async fn report_static(&self, _token: String, data: serde_json::Value) -> serde_json::Value {
        let parsed_data = match serde_json::from_value::<StaticMonitoringData>(data) {
            Ok(ok) => ok,
            Err(e) => {
                return generate_error_message(101, format!("Unable to parse json data: {}", e.to_string()));
            }
        };

        let in_data = static_monitoring::ActiveModel {
            id: Default::default(),
            uuid: Set(parsed_data.uuid),
            timestamp: Set(parsed_data.time as i64),

            cpu_data: Set(serde_json::value::to_value(parsed_data.cpu).unwrap()),
            system_data: Set(serde_json::value::to_value(parsed_data.system).unwrap()),
            gpu_data: Set(serde_json::value::to_value(parsed_data.gpu).unwrap()),
        };

        let db = match DB.get() {
            None => {
                return generate_error_message(102, "DB not initialized".to_string());
            }
            Some(db) => db,
        };

        let result = static_monitoring::Entity::insert(in_data)
            .exec(db)
            .await
            .unwrap();

        let new_id = result.last_insert_id;

        serde_json::json!({"id": new_id})
    }
}
