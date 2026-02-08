use crate::DB;
use crate::entity::crontab;
use crate::token::get::get_token;
use jsonrpsee::core::RpcResult;
use nodeget_lib::crontab::{Cron, CronType};
use nodeget_lib::permission::data_structure::{
    Crontab as CrontabPermission, Permission, Scope, Token,
};
use nodeget_lib::permission::token_auth::TokenOrAuth;
use nodeget_lib::utils::get_local_timestamp_ms;
use sea_orm::{DbErr, EntityTrait, RuntimeErr};
use serde_json::value::RawValue;
use std::collections::HashSet;
use uuid::Uuid;

pub async fn get(token: String) -> RpcResult<Box<RawValue>> {
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

        let has_crontab_read_permission = token_info.token_limit.iter().any(|limit| {
            limit
                .permissions
                .iter()
                .any(|perm| matches!(perm, Permission::Crontab(CrontabPermission::Read)))
        });

        if !has_crontab_read_permission {
            return Err((
                102,
                "Permission Denied: Insufficient Crontab Read permission".to_string(),
            ));
        }

        let crontabs = extract_allowed_uuids(&token_info).await?;
        let json_str = serde_json::to_string(&crontabs)
            .map_err(|e| (101, format!("Failed to serialize crontabs: {e}")))?;

        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}

async fn get_crontabs_by_uuids(uuids: Vec<Uuid>) -> Result<Vec<Cron>, DbErr> {
    let db = DB.get().ok_or(DbErr::Conn(RuntimeErr::Internal(
        "DB not initialized".to_string(),
    )))?;

    let models = crontab::Entity::find().all(db).await?;

    let uuid_set: HashSet<Uuid> = uuids.into_iter().collect();

    let crons = models
        .into_iter()
        .filter_map(|model| {
            let cron_type: CronType = serde_json::from_str(&model.cron_type.to_string())
                .unwrap_or({
                    CronType::Server(nodeget_lib::crontab::ServerCronType::CleanUpDatabase)
                });

            let should_include = match &cron_type {
                CronType::Agent(agent_uuids, _) => {
                    agent_uuids.iter().any(|uuid| uuid_set.contains(uuid))
                }
                CronType::Server(_) => false,
            };

            if should_include {
                Some(Cron {
                    id: model.id,
                    name: model.name,
                    enable: model.enable,
                    cron_expression: model.cron_expression,
                    cron_type,
                    last_run_time: model.last_run_time,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(crons)
}

async fn get_all_crontabs() -> Result<Vec<Cron>, DbErr> {
    let db = DB.get().ok_or(DbErr::Conn(RuntimeErr::Internal(
        "DB not initialized".to_string(),
    )))?;

    let models = crontab::Entity::find().all(db).await?;

    let crons = models
        .into_iter()
        .map(|model| {
            let cron_type: CronType = serde_json::from_str(&model.cron_type.to_string())
                .unwrap_or({
                    CronType::Server(nodeget_lib::crontab::ServerCronType::CleanUpDatabase)
                });
            Cron {
                id: model.id,
                name: model.name,
                enable: model.enable,
                cron_expression: model.cron_expression,
                cron_type,
                last_run_time: model.last_run_time,
            }
        })
        .collect();

    Ok(crons)
}

async fn extract_allowed_uuids(token_info: &Token) -> Result<Vec<Cron>, (i64, String)> {
    let mut has_global = false;
    let mut allowed_uuids: Vec<Uuid> = Vec::new();

    for limit in &token_info.token_limit {
        let has_crontab_read = limit
            .permissions
            .iter()
            .any(|p| matches!(p, Permission::Crontab(CrontabPermission::Read)));

        if !has_crontab_read {
            continue;
        }

        for scope in &limit.scopes {
            match scope {
                Scope::Global => {
                    has_global = true;
                }
                Scope::AgentUuid(uuid) => {
                    allowed_uuids.push(*uuid);
                }
            }
        }
    }

    if has_global {
        get_all_crontabs().await.map_err(|e| (103, e.to_string()))
    } else if !allowed_uuids.is_empty() {
        get_crontabs_by_uuids(allowed_uuids)
            .await
            .map_err(|e| (103, e.to_string()))
    } else {
        Ok(Vec::new())
    }
}
