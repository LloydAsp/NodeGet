// 该文件实现 供给调用者查询 API

use crate::entity::{dynamic_monitoring, static_monitoring};
use crate::rpc::RpcHelper;
use crate::rpc::agent::AgentRpcImpl;
use log::error;
use nodeget_lib::monitoring::query::{
    DynamicDataQuery, DynamicDataQueryField, DynamicResponseItem, QueryCondition, StaticDataQuery,
    StaticDataQueryField, StaticResponseItem,
};
use nodeget_lib::utils::error_message::generate_error_message;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, Order, QueryOrder, QuerySelect};
use serde_json::Value;

pub async fn query_static(_token: String, static_data_query: StaticDataQuery) -> Value {
    let process_logic = async {
        let db = AgentRpcImpl::get_db()?;

        // 查询构建器
        let mut query = static_monitoring::Entity::find();

        // 最新数据 (仅一个)
        let mut is_last = false;

        // 查询数量限制
        let mut limit_count: Option<u64> = None;

        // 应用过滤条件 (QueryCondition)
        for cond in static_data_query.condition {
            match cond {
                QueryCondition::Uuid(uuid) => {
                    query = query.filter(static_monitoring::Column::Uuid.eq(uuid));
                }
                QueryCondition::TimestampFromTo(start, end) => {
                    query = query.filter(
                        static_monitoring::Column::Timestamp
                            .gte(start)
                            .and(static_monitoring::Column::Timestamp.lte(end)),
                    );
                }
                QueryCondition::TimestampFrom(start) => {
                    query = query.filter(static_monitoring::Column::Timestamp.gte(start));
                }
                QueryCondition::TimestampTo(end) => {
                    query = query.filter(static_monitoring::Column::Timestamp.lte(end));
                }
                QueryCondition::Limit(n) => {
                    limit_count = Some(n);
                }
                QueryCondition::Last => {
                    is_last = true;
                }
            }
        }

        if let Some(l) = limit_count {
            query = query
                .order_by(static_monitoring::Column::Timestamp, Order::Desc)
                .limit(l);
        } else {
            query = query.order_by(static_monitoring::Column::Timestamp, Order::Asc);
        }

        // 时间倒序第一条
        if is_last {
            query = query
                .order_by(static_monitoring::Column::Timestamp, Order::Desc)
                .limit(1);
        } else {
            query = query.order_by(static_monitoring::Column::Timestamp, Order::Asc);
        }

        // 查询
        let models = query.all(db).await.map_err(|e| {
            error!("Database query error: {e}");
            (103, format!("Database query error: {e}"))
        })?;

        let result_list: Vec<StaticResponseItem> = models
            .into_iter()
            .map(|model| {
                let mut item = StaticResponseItem {
                    uuid: model.uuid.to_string(),
                    timestamp: model.timestamp,
                    cpu: None,
                    system: None,
                    gpu: None,
                };
                for field in &static_data_query.fields {
                    match field {
                        StaticDataQueryField::Cpu => item.cpu = Some(model.cpu_data.clone()),
                        StaticDataQueryField::System => {
                            item.system = Some(model.system_data.clone());
                        }
                        StaticDataQueryField::Gpu => item.gpu = Some(model.gpu_data.clone()),
                    }
                }
                item
            })
            .collect();
        Ok(serde_json::to_value(result_list).unwrap())
    };

    process_logic
        .await
        .unwrap_or_else(|(code, msg)| generate_error_message(code, &msg))
}

pub async fn query_dynamic(_token: String, dynamic_data_query: DynamicDataQuery) -> Value {
    let process_logic = async {
        let db = AgentRpcImpl::get_db()?;

        let mut query = dynamic_monitoring::Entity::find();

        let mut is_last = false;

        // 查询数量限制
        let mut limit_count: Option<u64> = None;

        for cond in dynamic_data_query.condition {
            match cond {
                QueryCondition::Uuid(uuid) => {
                    query = query.filter(dynamic_monitoring::Column::Uuid.eq(uuid));
                }
                QueryCondition::TimestampFromTo(start, end) => {
                    query = query.filter(
                        dynamic_monitoring::Column::Timestamp
                            .gte(start)
                            .and(dynamic_monitoring::Column::Timestamp.lte(end)),
                    );
                }
                QueryCondition::TimestampFrom(start) => {
                    query = query.filter(dynamic_monitoring::Column::Timestamp.gte(start));
                }
                QueryCondition::TimestampTo(end) => {
                    query = query.filter(dynamic_monitoring::Column::Timestamp.lte(end));
                }
                QueryCondition::Limit(n) => {
                    limit_count = Some(n);
                }
                QueryCondition::Last => {
                    is_last = true;
                }
            }
        }

        if let Some(l) = limit_count {
            query = query
                .order_by(dynamic_monitoring::Column::Timestamp, Order::Desc)
                .limit(l);
        } else {
            query = query.order_by(dynamic_monitoring::Column::Timestamp, Order::Asc);
        }

        if is_last {
            // 取最新的一条
            query = query
                .order_by(dynamic_monitoring::Column::Timestamp, Order::Desc)
                .limit(1);
        } else {
            // 默认按时间正序
            query = query.order_by(dynamic_monitoring::Column::Timestamp, Order::Asc);
        }

        let models = query.all(db).await.map_err(|e| {
            error!("Database query error: {e}");
            (103, format!("Database query error: {e}"))
        })?;

        let result_list: Vec<DynamicResponseItem> = models
            .into_iter()
            .map(|model| {
                let mut item = DynamicResponseItem {
                    uuid: model.uuid.to_string(),
                    timestamp: model.timestamp,
                    cpu: None,
                    ram: None,
                    load: None,
                    system: None,
                    disk: None,
                    network: None,
                    gpu: None,
                };
                for field in &dynamic_data_query.fields {
                    match field {
                        DynamicDataQueryField::Cpu => item.cpu = Some(model.cpu_data.clone()),
                        DynamicDataQueryField::Ram => item.ram = Some(model.ram_data.clone()),
                        DynamicDataQueryField::Load => item.load = Some(model.load_data.clone()),
                        DynamicDataQueryField::System => {
                            item.system = Some(model.system_data.clone());
                        }
                        DynamicDataQueryField::Disk => item.disk = Some(model.disk_data.clone()),
                        DynamicDataQueryField::Network => {
                            item.network = Some(model.network_data.clone());
                        }
                        DynamicDataQueryField::Gpu => item.gpu = Some(model.gpu_data.clone()),
                    }
                }
                item
            })
            .collect();
        Ok(serde_json::to_value(result_list).unwrap())
    };

    process_logic
        .await
        .unwrap_or_else(|(code, msg)| generate_error_message(code, &msg))
}
