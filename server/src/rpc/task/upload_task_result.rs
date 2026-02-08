use crate::entity::task;
use crate::rpc::RpcHelper;
use crate::token::get::check_token_limit;
use jsonrpsee::core::RpcResult;
use log::{debug, error};
use nodeget_lib::permission::data_structure::{Permission, Scope, Task};
use nodeget_lib::permission::token_auth::TokenOrAuth;
use nodeget_lib::task::{TaskEventResponse, TaskEventType};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde_json::value::RawValue;
use serde_json::Value;

pub async fn upload_task_result(
    token: String,
    task_response: TaskEventResponse,
) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let db = <super::TaskRpcImpl as RpcHelper>::get_db()?;

        let task_model = task::Entity::find_by_id(task_response.task_id.cast_signed())
            .filter(task::Column::Uuid.eq(task_response.agent_uuid))
            .filter(task::Column::Token.eq(task_response.task_token.clone()))
            .one(db)
            .await
            .map_err(|e| {
                error!("Database query error: {e}");
                (103, format!("Database query error: {e}"))
            })?
            .ok_or_else(|| {
                (
                    105,
                    "Task validation failed: Invalid ID, UUID, or Token".to_string(),
                )
            })?;

        let original_task_type: TaskEventType =
            serde_json::from_value(task_model.task_event_type.clone())
                .map_err(|e| (101, format!("Failed to parse original task type: {e}")))?;

        let task_name = original_task_type.task_name();

        let token_or_auth = match TokenOrAuth::from_full_token(&token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        let is_allowed = check_token_limit(
            &token_or_auth,
            vec![Scope::AgentUuid(task_response.agent_uuid)],
            vec![Permission::Task(Task::Write(task_name.to_string()))],
        )
        .await?;

        if !is_allowed {
            return Err((
                102,
                format!(
                    "Permission Denied: Missing Task Write ({task_name}) permission for this Agent"
                ),
            ));
        }

        let mut active_model: task::ActiveModel = task_model.into();

        active_model.timestamp = Set(Some(task_response.timestamp.cast_signed()));
        active_model.success = Set(Some(task_response.success));

        active_model.error_message = Set(task_response.error_message.map(|v| {
            let json_v = serde_json::to_value(v).unwrap_or(Value::Null);
            match json_v {
                Value::String(s) => s,
                _ => json_v.to_string(),
            }
        }));

        let result_json = task_response
            .task_event_result
            .map(<super::TaskRpcImpl as RpcHelper>::try_set_json)
            .transpose()
            .map_err(|e| (101, e))?;

        active_model.task_event_result =
            result_json.map_or(Set(None), |active_val| Set(Some(active_val.unwrap())));

        active_model.update(db).await.map_err(|e| {
            error!("Database update error: {e}");
            (103, format!("Database update error: {e}"))
        })?;

        debug!(
            "Task [{}] result uploaded successfully by auth identifying as {:?}",
            task_response.task_id,
            if token_or_auth.is_auth() {
                "Auth"
            } else {
                "Token"
            }
        );

        let json_str = format!("{{\"id\":{}}}", task_response.task_id);
        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
