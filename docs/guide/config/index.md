# 配置文件

Nodeget 均使用 Toml 作为配置文件格式，请事先了解 Toml 语法规范: <https://toml.io/cn/>

程序在启动时会自动读取 `./config.toml`，若无法读取将会 Panic 退出

- [Agent](./agent.md)
- [Server](./server.md)