# nodeget-server_hello

测试服务是否正常运行。

## 请求

```json
{
  "jsonrpc": "2.0",
  "method": "nodeget-server_hello",
  "params": [],
  "id": 1
}
```

## 响应

```json
{
  "jsonrpc": "2.0",
  "result": "NodeGet Server Is Running!",
  "id": 1
}
```

## 返回值

| 类型 | 描述 |
|------|------|
| `String` | 返回字符串 `"NodeGet Server Is Running!"` 表示服务正常运行 |

## 使用场景

- 健康检查
- 服务可用性测试
- 连接测试
