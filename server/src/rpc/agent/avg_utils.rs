use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
enum JsonAvgNode {
    #[default]
    Null,
    Number { sum: f64, count: u64 },
    Object(BTreeMap<String, JsonAvgNode>),
    Array(Vec<JsonAvgNode>),
}

#[derive(Debug, Clone, Default)]
pub struct JsonAverageAccumulator {
    root: JsonAvgNode,
}

impl JsonAverageAccumulator {
    pub fn add(&mut self, value: &Value) {
        merge_node(&mut self.root, value);
    }

    #[must_use]
    pub fn finalize(&self) -> Value {
        node_to_value(&self.root)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProcessCountAverageAccumulator {
    sum: f64,
    count: u64,
}

impl ProcessCountAverageAccumulator {
    pub fn add(&mut self, value: &Value) {
        let Some(process_count) = value
            .as_object()
            .and_then(|obj| obj.get("process_count"))
            .and_then(Value::as_f64)
        else {
            return;
        };

        if process_count.is_finite() {
            self.sum += process_count;
            self.count += 1;
        }
    }

    #[must_use]
    pub fn finalize(&self) -> Value {
        let mut obj = Map::new();
        let value = if self.count == 0 {
            Value::Null
        } else {
            number_value(self.sum / self.count as f64)
        };
        obj.insert("process_count".to_owned(), value);
        Value::Object(obj)
    }
}

fn merge_node(node: &mut JsonAvgNode, value: &Value) {
    match value {
        Value::Number(number) => {
            let Some(v) = number.as_f64() else {
                return;
            };
            if !v.is_finite() {
                return;
            }
            match node {
                JsonAvgNode::Null => {
                    *node = JsonAvgNode::Number { sum: v, count: 1 };
                }
                JsonAvgNode::Number { sum, count } => {
                    *sum += v;
                    *count += 1;
                }
                JsonAvgNode::Object(_) | JsonAvgNode::Array(_) => {}
            }
        }
        Value::Object(map) => {
            let target = match node {
                JsonAvgNode::Null => {
                    *node = JsonAvgNode::Object(BTreeMap::new());
                    match node {
                        JsonAvgNode::Object(inner) => inner,
                        _ => unreachable!(),
                    }
                }
                JsonAvgNode::Object(inner) => inner,
                JsonAvgNode::Number { .. } | JsonAvgNode::Array(_) => return,
            };

            for (key, item) in map {
                let child = target.entry(key.clone()).or_insert(JsonAvgNode::Null);
                merge_node(child, item);
            }
        }
        Value::Array(items) => {
            let target = match node {
                JsonAvgNode::Null => {
                    *node = JsonAvgNode::Array(Vec::new());
                    match node {
                        JsonAvgNode::Array(inner) => inner,
                        _ => unreachable!(),
                    }
                }
                JsonAvgNode::Array(inner) => inner,
                JsonAvgNode::Number { .. } | JsonAvgNode::Object(_) => return,
            };

            if target.len() < items.len() {
                target.resize(items.len(), JsonAvgNode::Null);
            }

            for (index, item) in items.iter().enumerate() {
                merge_node(&mut target[index], item);
            }
        }
        Value::String(_) | Value::Bool(_) | Value::Null => {}
    }
}

fn node_to_value(node: &JsonAvgNode) -> Value {
    match node {
        JsonAvgNode::Null => Value::Null,
        JsonAvgNode::Number { sum, count } => {
            if *count == 0 {
                Value::Null
            } else {
                number_value(*sum / *count as f64)
            }
        }
        JsonAvgNode::Object(map) => {
            let mut obj = Map::with_capacity(map.len());
            for (key, value) in map {
                obj.insert(key.clone(), node_to_value(value));
            }
            Value::Object(obj)
        }
        JsonAvgNode::Array(items) => Value::Array(items.iter().map(node_to_value).collect()),
    }
}

fn number_value(value: f64) -> Value {
    if !value.is_finite() {
        return Value::Null;
    }
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}
