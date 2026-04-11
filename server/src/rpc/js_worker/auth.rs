use crate::token::get::check_token_limit;
use nodeget_lib::error::NodegetError;
use nodeget_lib::permission::data_structure::{
    JsWorker as JsWorkerPermission, NodeGet, Permission, Scope,
};
use nodeget_lib::permission::token_auth::TokenOrAuth;
use tracing::{trace, warn};

pub async fn check_js_worker_permission(
    token: &str,
    worker_name: &str,
    permission: JsWorkerPermission,
) -> anyhow::Result<()> {
    trace!(target: "js_worker", worker_name = %worker_name, permission = ?permission, "checking js_worker permission");
    let token_or_auth = TokenOrAuth::from_full_token(token)
        .map_err(|e| NodegetError::ParseError(format!("Failed to parse token: {e}")))?;

    let permission_name = format!("{permission:?}");
    let is_allowed = check_token_limit(
        &token_or_auth,
        vec![Scope::JsWorker(worker_name.to_owned())],
        vec![Permission::JsWorker(permission)],
    )
    .await?;

    if is_allowed {
        return Ok(());
    }

    warn!(target: "js_worker", worker_name = %worker_name, permission = %permission_name, "permission denied");
    Err(NodegetError::PermissionDenied(format!(
        "Permission denied for js_worker '{worker_name}', required permission: {permission_name}"
    ))
    .into())
}

pub async fn check_get_rt_pool_permission(token: &str) -> anyhow::Result<()> {
    trace!(target: "js_worker", "checking get_rt_pool permission");
    let token_or_auth = TokenOrAuth::from_full_token(token)
        .map_err(|e| NodegetError::ParseError(format!("Failed to parse token: {e}")))?;

    let is_allowed = check_token_limit(
        &token_or_auth,
        vec![Scope::Global],
        vec![Permission::NodeGet(NodeGet::GetRtPool)],
    )
    .await?;

    if is_allowed {
        return Ok(());
    }

    warn!(target: "js_worker", "get_rt_pool permission denied");
    Err(NodegetError::PermissionDenied(
        "Permission denied: missing nodeget.get_rt_pool permission".to_owned(),
    )
    .into())
}

pub async fn filter_workers_by_list_permission(
    token: &str,
    worker_names: Vec<String>,
) -> anyhow::Result<Vec<String>> {
    trace!(target: "js_worker", count = worker_names.len(), "filtering workers by list permission");
    let token_or_auth = TokenOrAuth::from_full_token(token)
        .map_err(|e| NodegetError::ParseError(format!("Failed to parse token: {e}")))?;

    let mut allowed = Vec::new();
    for worker_name in worker_names {
        let is_allowed = check_token_limit(
            &token_or_auth,
            vec![Scope::JsWorker(worker_name.clone())],
            vec![Permission::JsWorker(JsWorkerPermission::ListAllJsWorker)],
        )
        .await?;

        if is_allowed {
            allowed.push(worker_name);
        }
    }

    Ok(allowed)
}
