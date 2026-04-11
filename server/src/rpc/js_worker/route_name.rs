use nodeget_lib::error::NodegetError;
use tracing::warn;

pub fn normalize_route_name(route_name: Option<String>) -> anyhow::Result<Option<String>> {
    let Some(raw) = route_name else {
        return Ok(None);
    };

    let normalized = raw.trim().to_owned();
    if normalized.is_empty() {
        warn!(target: "js_worker", "route_name validation failed: empty string");
        return Err(
            NodegetError::InvalidInput("route_name cannot be empty string".to_owned()).into(),
        );
    }

    if !normalized
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        warn!(target: "js_worker", route_name = %normalized, "route_name validation failed: invalid characters");
        return Err(NodegetError::InvalidInput(
            "route_name can only contain [a-zA-Z0-9._-]".to_owned(),
        )
        .into());
    }

    Ok(Some(normalized))
}
