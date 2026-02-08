use crate::entity::dynamic_monitoring;
use crate::rpc::RpcHelper;
use crate::rpc::agent::AgentRpcImpl;
use crate::token::get::check_token_limit;
use jsonrpsee::core::RpcResult;
use log::debug;
use nodeget_lib::monitoring::data_structure::DynamicMonitoringData;
use nodeget_lib::permission::data_structure::{DynamicMonitoring, Permission, Scope};
use nodeget_lib::permission::token_auth::TokenOrAuth;
use sea_orm::{ActiveValue, EntityTrait, Set};
use serde_json::value::RawValue;
use std::str::FromStr;

pub async fn report_dynamic(
    token: String,
    dynamic_monitoring_data: DynamicMonitoringData,
) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let agent_uuid = uuid::Uuid::from_str(&dynamic_monitoring_data.uuid)
            .map_err(|e| (101, format!("Invalid UUID format: {e}")))?;

        let token_or_auth = match TokenOrAuth::from_full_token(&token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        let is_allowed = check_token_limit(
            &token_or_auth,
            vec![Scope::AgentUuid(agent_uuid)],
            vec![Permission::DynamicMonitoring(DynamicMonitoring::Write)],
        )
        .await?;

        if !is_allowed {
            return Err((
                102,
                "Permission Denied: Missing DynamicMonitoring Write permission for this Agent"
                    .to_string(),
            ));
        }

        let db = AgentRpcImpl::get_db()?;

        let in_data = dynamic_monitoring::ActiveModel {
            id: ActiveValue::default(),
            uuid: Set(agent_uuid),
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
            dynamic_monitoring_data.uuid
        );

        let result = dynamic_monitoring::Entity::insert(in_data)
            .exec(db)
            .await
            .map_err(|e| {
                log::error!("Database insert error: {e}");
                (103, format!("Database insert error: {e}"))
            })?;

        debug!("Inserted dynamic data with id [{}]", result.last_insert_id);

        let json_str = format!("{{\"id\":{}}}", result.last_insert_id);
        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
