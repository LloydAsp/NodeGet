# 调用者查询

调用者可以通过 `agent_query_static(dynamic)` 查询

需要传入 `token` / `static(dynamic)_data_query`:

```json
{
    "token": "demo_token",
    "static(dynamic)_data_query": {
        "fields": [
            // DataQueryField 结构体，该结构体参考 Monitoring 总览
            // 该字段为 Vec<_>，可指定多个
        ],
        "condition": [
            // QueryCondition 结构体，该结构体参考 Monitoring 总览
            // 该字段为 Vec<_>，可指定多个
        ]
    }
}
```

返回结构:

```json
[
    // Monitoring 回报结构体，该结构体参考 Monitoring 总览
    // 该字段为 Vec<_>，可指定多个
    // 只会存在 DataQueryField 结构中指定的数据字段
]
```

## 批量获取多个 Agent 的最新数据

为了便于直接查询多个 Agent 的最新一条监控数据，新增了两个方法：

- `agent_static_data_multi_last_query`
- `agent_dynamic_data_multi_last_query`

这两个方法等价于原 `agent_query_static(dynamic)` 中为每个 UUID 设置 `condition last` 的效果，但调用更直接。

需要传入 `token` / `uuids` / `fields`：

```json
{
    "token": "demo_token",
    "uuids": [
        "e8583352-39e8-5a5b-b66c-e450689088fd",
        "830cec66-8fc9-5c21-9e2d-2da2b2f2d3b3"
    ],
    "fields": [
        // DataQueryField 结构体，该结构体参考 Monitoring 总览
        // 该字段为 Vec<_>，可指定多个
    ]
}
```

返回结构:

```json
[
    // 每个 UUID 最多返回一条最新数据
    // 固定包含 uuid / timestamp
    // 只会包含 fields 指定的数据字段
]
```
