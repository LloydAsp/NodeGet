use crate::entity::metadata as metadata_entity;
use crate::rpc::RpcHelper;
use crate::token::get::check_token_limit;
use jsonrpsee::core::RpcResult;
use nodeget_lib::metadata;
use nodeget_lib::permission::data_structure::{Metadata as MetadataPermission, Permission, Scope};
use nodeget_lib::permission::token_auth::TokenOrAuth;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter,
};
use serde_json::value::RawValue;

pub async fn write(token: String, metadata: metadata::Metadata) -> RpcResult<Box<RawValue>> {
    let process_logic = async {
        let token_or_auth = match TokenOrAuth::from_full_token(&token) {
            Ok(toa) => toa,
            Err(e) => return Err((101, format!("Failed to parse token: {e}"))),
        };

        let is_allowed = check_token_limit(
            &token_or_auth,
            vec![Scope::AgentUuid(metadata.agent_uuid)],
            vec![Permission::Metadata(MetadataPermission::Write)],
        )
        .await?;

        if !is_allowed {
            return Err((
                102,
                "Permission Denied: Missing Metadata Write permission".to_string(),
            ));
        }

        let db = <super::MetadataRpcImpl as RpcHelper>::get_db()?;

        let tags_json = serde_json::to_value(&metadata.agent_tags)
            .map_err(|e| (101, format!("Failed to serialize tags: {e}")))?;

        let id = match metadata_entity::Entity::find()
            .filter(metadata_entity::Column::Uuid.eq(metadata.agent_uuid))
            .one(db)
            .await
        {
            Ok(Some(existing_model)) => {
                let mut active_model: metadata_entity::ActiveModel = existing_model.into();
                active_model.name = ActiveValue::Set(metadata.agent_name);
                active_model.tags = ActiveValue::Set(Some(tags_json));

                active_model
                    .update(db)
                    .await
                    .map_err(|e| (103, format!("Database update error: {e}")))?
                    .id
            }
            Ok(None) => {
                let new_metadata = metadata_entity::ActiveModel {
                    id: ActiveValue::default(),
                    uuid: ActiveValue::Set(metadata.agent_uuid),
                    name: ActiveValue::Set(metadata.agent_name),
                    tags: ActiveValue::Set(Some(tags_json)),
                };

                new_metadata
                    .insert(db)
                    .await
                    .map_err(|e| (103, format!("Database insert error: {e}")))?
                    .id
            }
            Err(e) => return Err((103, format!("Database error: {e}"))),
        };

        let json_str = format!("{{\"id\":{}}}", id);

        RawValue::from_string(json_str)
            .map_err(|e| (101, e.to_string()))
    };

    process_logic
        .await
        .map_err(|(code, msg)| jsonrpsee::types::ErrorObject::owned(code as i32, msg, None::<()>))
}
