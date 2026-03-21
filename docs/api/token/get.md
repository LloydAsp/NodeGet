# 获取 Token 详情

提供一个 Token，即可获取在 `Token 总览` 中的 Token 对应结构体

## 获取方法

`token_get` 是用于获取的方法，需要提供:

- `token`: 需要查询的 Token
- `supertoken`（可选）: SuperToken，启用后允许 `token` 传入 `username` / `token_key`

```json
{
  "token": "demo_token"
}
```

当你持有 SuperToken 时，可以用简写查询指定 Token：

```json
{
  "token": "target_username_or_token_key",
  "supertoken": "SUPER_TOKEN_KEY:SUPER_TOKEN_SECRET"
}
```

若 `supertoken` 存在，`token` 仍然支持完整格式（`token_key:token_secret` 或 `username|password`）。

## 返回值

返回值即为 `Token 总览` 中的 Token 结构体:

```json
{
  "timestamp_from": null,
  "timestamp_to": null,
  "token_key": "n0kB8lSAykFd9Egu",
  "token_limit": [
    {
      "permissions": [
        {
          "task": "listen"
        },
        {
          "task": {
            "write": "ping"
          }
        },
        {
          "task": {
            "create": "ping"
          }
        },
        {
          "task": {
            "create": "tcp_ping"
          }
        }
      ],
      "scopes": [
        "global"
      ]
    }
  ],
  "username": null,
  "version": 1
}
```

当 Token 具有 Crontab 权限时，返回值中可能会包含类似以下的权限信息：

```json
{
  "permissions": [
    {
      "crontab": "read"
    },
    {
      "crontab": "write"
    },
    {
      "crontab": "delete"
    }
  ]
}
```
