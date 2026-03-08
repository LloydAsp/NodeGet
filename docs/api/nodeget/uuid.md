# nodeget-server_uuid

获取当前 Server 的 UUID。

## 方法

调用方法名为 `nodeget-server_uuid`。

该方法不需要 `token`，也不需要鉴权。

## 请求

```json
{
  "jsonrpc": "2.0",
  "method": "nodeget-server_uuid",
  "params": [],
  "id": 1
}
```

## 响应

```json
{
  "jsonrpc": "2.0",
  "result": "e8583352-39e8-5a5b-b66c-e450689088fd",
  "id": 1
}
```
