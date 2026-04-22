# NodeGet worker的架构

NodeGet Worker = `quickjs` + `llrt library` + `NodeGet extension / bind`

NodeGet Worker是 NodeGet 嵌入的 Js 运行时(runtime)，其基础来自于 quickjs 项目。

选择 quickjs 而非v8引擎的原因是: quickjs 具有快速的启动时间和极小的内存足迹和打包体积，非常适合作为探针的嵌入式运行时。

同时为了在 Js Worker内方便调用 NodeGet 项目的接口，提供了名为 nodeget 的全局函数

为了支持标准js语法之外的扩展功能，借助了 aws 的 llrt 项目，补充实现了下面的模块
- fetch/Request/Response/Headers/FormData
- atob/btoa/TextEncoder/TextDecoder
- URL/URLSearchParams
- setTimeout/clearTimeout/setInterval

关于 NodeGet Worker 的详细能力扩展说明，可以参考[API](/api/js_worker/injected)

此外，我们扩展了更多的调用入口，比如 onCall / onCron / onHttp / onInlineCall，使其能够和 NodeGet 项目无缝结合

```
export default {
  // 通过 JSONRpc调用
  async onCall(params, env, ctx) {
    return { ok: true, from: "onCall", params, env };
  },

  // 通过 worker 相互调用
  async onInlineCall(params, env, ctx) {
    return { ok: true, from: "onInlineCall", params, env };
  },

  // 通过 cron、 JSONRpc调用
  async onCron(params, env, ctx) {
    return { ok: true, from: "onCron", params, env };
  },

  // 通过 http请求、 JSONRpc调用
  async onRoute(request, env, ctx) {
    return new Response("ok", { status: 200 });
  }
};
```

这些特殊函数如何工作，可以参考 [代码规范](./coding-guide) 和 [API](/api/js_worker/script)


## Worker 的调用关系
下面简单介绍下 Worker 支持的调用关系

### api 调用 Worker
通过 JSON-RPC 发起 js-worker_run，可以执行 onCall 函数，为了方便开发调试，这个函数也支持模拟触发onCron、onRoute

并提供相关的 params 变量

### Worker 调用api
可以通过 worker 内的 nodeget函数，调用所有的 NodeGet接口，这个是跳过了 websocket 请求，直接触发对应的行为逻辑

虽然是没有网络数据包，但所需要的鉴权token并不会跳过，对于不同的api，仍然需要提供所需的token

Worker 只是行为，不代表权限。

### Worker 调用 Worker

Worker 自建可以通过 inlineCall 入口相互调用，可以通过 ctx.inlineCaller 获得调用者并决定是否继续执行

### http 路由绑定

Worker 可以绑定 http 路由，进而实现与外部系统交互，比如各种 webhook，具体的应用如：
- Telegram 机器人可以完整部署到 NodeGet Worker 上
- GitHub 更新的 webhook

## Worker的运行模式
下面是 NodeGet 在实现 Worker 时采取的一些考量
- 每个 Worker 的代码被更新时会预编译为字节码提高运行效率
- 一个 Worker 对应一个 Runtime 实例，Worker 间相互隔离
- Runtime 长时间不使用会被清理，可以设定每隔 Runtime 的不活跃清理时间
- 每个worker储存他们自己的 env 变量，在函数运行时会被注入