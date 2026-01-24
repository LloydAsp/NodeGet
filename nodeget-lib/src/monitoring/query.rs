use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaticDataQueryField {
    Cpu,
    System,
    Gpu,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicDataQueryField {
    Cpu,
    Ram,
    Load,
    System,
    Disk,
    Network,
    Gpu,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryCondition {
    Uuid(uuid::Uuid),
    TimestampFromTo(i64, i64), // start, end
    TimestampFrom(i64),        // start,
    TimestampTo(i64),          // end

    Limit(u64), // limit

    Last,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaticDataQuery {
    pub fields: Vec<StaticDataQueryField>,
    pub condition: Vec<QueryCondition>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicDataQuery {
    pub fields: Vec<DynamicDataQueryField>,
    pub condition: Vec<QueryCondition>,
}

#[derive(Serialize)]
pub struct StaticResponseItem {
    pub uuid: String,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<Value>,
}

#[derive(Serialize)]
pub struct DynamicResponseItem {
    pub uuid: String,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<Value>,
}
