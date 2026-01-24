use crate::entity::{dynamic_monitoring, static_monitoring};
use crate::rpc::RpcHelper;
use crate::rpc::agent::AgentRpcImpl;
use log::{debug, error};
use nodeget_lib::monitoring::data_structure::{DynamicMonitoringData, StaticMonitoringData};
use nodeget_lib::utils::error_message::generate_error_message;
use sea_orm::{ActiveValue, EntityTrait, Set};
use serde_json::{Value, json};
use std::str::FromStr;

pub async fn report_static(_token: String, static_monitoring_data: StaticMonitoringData) -> Value {
    let process_logic = async {
        let db = AgentRpcImpl::get_db()?;

        let in_data = static_monitoring::ActiveModel {
            id: ActiveValue::default(),
            uuid: Set(uuid::Uuid::from_str(&static_monitoring_data.uuid)
                .map_err(|e| (101, e.to_string()))?),
            timestamp: Set(static_monitoring_data.time.cast_signed()),

            cpu_data: AgentRpcImpl::try_set_json(static_monitoring_data.cpu)
                .map_err(|e| (101, e))?,
            system_data: AgentRpcImpl::try_set_json(static_monitoring_data.system)
                .map_err(|e| (101, e))?,
            gpu_data: AgentRpcImpl::try_set_json(static_monitoring_data.gpu)
                .map_err(|e| (101, e))?,
        };

        debug!(
            "Received static data from [{}]",
            static_monitoring_data.uuid.clone()
        );

        let result = static_monitoring::Entity::insert(in_data)
            .exec(db)
            .await
            .map_err(|e| {
                error!("Database insert error: {e}");
                (103, format!("Database insert error: {e}"))
            })?;

        debug!("Inserted static data with id [{}]", result.last_insert_id);

        Ok(result.last_insert_id)
    };

    match process_logic.await {
        Ok(new_id) => json!({ "id": new_id }),
        Err((code, msg)) => generate_error_message(code, &msg),
    }
}

pub async fn report_dynamic(
    _token: String,
    dynamic_monitoring_data: DynamicMonitoringData,
) -> Value {
    let process_logic = async {
        let db = AgentRpcImpl::get_db()?;

        let in_data = dynamic_monitoring::ActiveModel {
            id: ActiveValue::default(),
            uuid: Set(uuid::Uuid::from_str(&dynamic_monitoring_data.uuid)
                .map_err(|e| (101, e.to_string()))?),
            timestamp: Set(dynamic_monitoring_data.time.cast_signed()),

            cpu_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.cpu)
                .map_err(|e| (101, e))?,
            ram_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.ram)
                .map_err(|e| (101, e))?,
            load_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.load)
                .map_err(|e| (101, e))?,
            system_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.system)
                .map_err(|e| (101, e))?,
            disk_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.disk)
                .map_err(|e| (101, e))?,
            network_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.network)
                .map_err(|e| (101, e))?,
            gpu_data: AgentRpcImpl::try_set_json(dynamic_monitoring_data.gpu)
                .map_err(|e| (101, e))?,
        };

        debug!(
            "Received dynamic data from [{}]",
            dynamic_monitoring_data.uuid.clone()
        );

        let result = dynamic_monitoring::Entity::insert(in_data)
            .exec(db)
            .await
            .map_err(|e| {
                error!("Database insert error: {e}");
                (103, format!("Database insert error: {e}"))
            })?;

        debug!("Inserted dynamic data with id [{}]", result.last_insert_id);

        Ok(result.last_insert_id)
    };

    match process_logic.await {
        Ok(new_id) => json!({ "id": new_id }),
        Err((code, msg)) => generate_error_message(code, &msg),
    }
}
