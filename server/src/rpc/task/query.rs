use crate::entity::task;
use crate::rpc::RpcHelper;
use crate::rpc::task::TaskRpcImpl;
use log::error;
use nodeget_lib::task::query::{TaskDataQuery, TaskQueryCondition, TaskResponseItem};
use nodeget_lib::utils::error_message::generate_error_message;
use sea_orm::sea_query::{Alias, BinOper, Expr};
use sea_orm::{
    ColumnTrait, DbBackend, EntityTrait, ExprTrait, Order, QueryFilter, QueryOrder, QuerySelect,
};
use serde_json::Value;

pub async fn query(_token: String, task_data_query: TaskDataQuery) -> Value {
    let process_logic = async {
        let db = TaskRpcImpl::get_db()?;

        let mut query = task::Entity::find();
        let mut is_last = false;

        let mut limit_count: Option<u64> = None;

        for cond in task_data_query.condition {
            match cond {
                TaskQueryCondition::TaskId(id) => {
                    query = query.filter(task::Column::Id.eq(id.cast_signed()));
                }

                TaskQueryCondition::Uuid(uuid) => {
                    query = query.filter(task::Column::Uuid.eq(uuid));
                }
                TaskQueryCondition::TimestampFromTo(start, end) => {
                    query = query.filter(
                        task::Column::Timestamp
                            .gte(start)
                            .and(task::Column::Timestamp.lte(end)),
                    );
                }
                TaskQueryCondition::TimestampFrom(start) => {
                    query = query.filter(task::Column::Timestamp.gte(start));
                }
                TaskQueryCondition::TimestampTo(end) => {
                    query = query.filter(task::Column::Timestamp.lte(end));
                }
                TaskQueryCondition::IsSuccess => {
                    query = query.filter(task::Column::Success.eq(true));
                }
                TaskQueryCondition::IsFailure => {
                    query = query.filter(task::Column::Success.eq(false));
                }
                TaskQueryCondition::IsRunning => {
                    query = query.filter(task::Column::Success.is_null());
                }
                TaskQueryCondition::Type(type_key) => {
                    if db.get_database_backend() == DbBackend::Postgres {
                        // Postgres 优化：使用 JSONB 操作符
                        query = query.filter(
                            Expr::col(task::Column::TaskEventType)
                                .binary(BinOper::Custom("?"), type_key),
                        );
                    } else {
                        // SQLite / 其他，转文本并匹配
                        let pattern = format!("%\"{type_key}\":%");
                        query = query.filter(
                            Expr::col(task::Column::TaskEventType)
                                .cast_as(Alias::new("text"))
                                .like(pattern),
                        );
                    }
                }

                TaskQueryCondition::Limit(n) => {
                    limit_count = Some(n);
                }

                TaskQueryCondition::Last => {
                    is_last = true;
                }
            }
        }

        if let Some(l) = limit_count {
            query = query.order_by(task::Column::Id, Order::Desc).limit(l);
        } else {
            query = query.order_by(task::Column::Id, Order::Asc);
        }

        if is_last {
            query = query.order_by(task::Column::Id, Order::Desc).limit(1);
        } else {
            query = query.order_by(task::Column::Id, Order::Asc);
        }

        let models = query.all(db).await.map_err(|e| {
            error!("Database query error: {e}");
            (103, format!("Database query error: {e}"))
        })?;

        let result_list: Vec<TaskResponseItem> = models
            .into_iter()
            .map(|model| TaskResponseItem {
                task_id: 0,
                uuid: model.uuid.to_string(),
                timestamp: model.timestamp,
                success: model.success,
                task_event_type: model.task_event_type,
                task_event_result: model.task_event_result,
                error_message: model.error_message,
            })
            .collect();
        Ok(serde_json::to_value(result_list).unwrap())
    };

    process_logic
        .await
        .unwrap_or_else(|(code, msg)| generate_error_message(code, &msg))
}
