use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskQueryCondition {
    TaskId(u64),
    Uuid(uuid::Uuid),
    TimestampFromTo(i64, i64), // start, end
    TimestampFrom(i64),        // start,
    TimestampTo(i64),          // end

    IsSuccess,    // 仅查找 success 字段为 true
    IsFailure,    // 仅查找 success 字段为 false
    IsRunning,    // 仅查找 success 字段为空
    Type(String), // task_event_type 中有字段为 `String` 的行

    Limit(u64), // limit

    Last,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskDataQuery {
    pub condition: Vec<TaskQueryCondition>,
}

#[derive(Serialize)]
pub struct TaskResponseItem {
    pub task_id: i64,
    pub uuid: String,
    pub timestamp: Option<i64>,
    pub success: Option<bool>,
    pub task_event_type: Value,
    pub task_event_result: Option<Value>,
    pub error_message: Option<String>,
}
