# nodeget-server_version

获取服务器版本信息。

## 请求

```json
{
  "jsonrpc": "2.0",
  "method": "nodeget-server_version",
  "params": [],
  "id": 1
}
```

## 响应

```json
{
  "jsonrpc": "2.0",
  "result": {
    "version": "0.1.0",
    "build_date": "2024-01-01",
    "git_commit": "abc123"
  },
  "id": 1
}
```

## 返回值

| 字段 | 类型 | 描述 |
|------|------|------|
| `version` | `String` | 版本号 |
| `build_date` | `String` | 构建日期 |
| `git_commit` | `String` | Git 提交哈希 |

## 使用场景

- 版本兼容性检查
- 调试信息收集
- 服务信息展示
