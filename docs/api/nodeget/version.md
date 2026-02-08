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
    "binary_type": "Server",
    "build_time": "2026-02-08T10:44:02.848471700Z",
    "cargo_target_triple": "x86_64-pc-windows-msvc",
    "cargo_version": "0.0.1",
    "git_branch": "main",
    "git_commit_date": "2026-02-08T07:25:09.000000000Z",
    "git_commit_message": "Feat: 优化 Task Name 对应关系Docs: 新增 Hello Version",
    "git_commit_sha": "73d9589",
    "rustc_channel": "nightly",
    "rustc_commit_date": "2025-12-30",
    "rustc_commit_hash": "0e8999942552691afc20495af6227eca8ab0af05",
    "rustc_llvm_version": "21.1",
    "rustc_version": "1.94.0-nightly"
}
```