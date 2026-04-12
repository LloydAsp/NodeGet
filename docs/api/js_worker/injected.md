# JS Runtime 外部注入能力

注入入口：`server/src/js_runtime/mod.rs` 的 `init_js_runtime_globals`。

## 自定义注入

### 全局函数

- `globalThis.nodeget(json)` — 调用 NodeGet JSON-RPC API，`json` 可以是 JSON 字符串或 JS 对象（对象会自动 `JSON.stringify`），返回解析后的 JS 对象
- `globalThis.inlineCall(js_worker_name, params, timeout_sec?)` — 调用其他 JS Worker
- `globalThis.randomUUID()` — 生成随机 UUID v4 字符串

### runtimeCtx（handler 第三参数）

脚本 handler 签名为 `handler(input, env, runtimeCtx)`，其中 `runtimeCtx` 包含以下属性：

- `runtimeCtx.runType` — 当前运行类型字符串：`"onCall"` / `"onCron"` / `"onRoute"` / `"onInlineCall"`
- `runtimeCtx.workerName` — 当前 Worker 的名字
- `runtimeCtx.inlineCall(js_worker_name, params, timeout_sec?)` — 等价于 `globalThis.inlineCall`
- `runtimeCtx.inlineCaller` — 调用当前脚本的调用者脚本名；顶层调用时为 `null`

## llrt_* 模块支持

- `llrt_fetch::init`
    - `fetch`、`Request`、`Response`、`Headers`、`FormData`
- `llrt_buffer::init`
    - `Buffer`、`Blob`、`File`、`atob`、`btoa`
- `llrt_stream_web::init`
    - `ReadableStream`、`WritableStream`、`TransformStream`
- `llrt_url::init`
    - `URL`、`URLSearchParams`
- `llrt_util::init`
    - `TextEncoder`、`TextDecoder`
- `llrt_timers::init`
    - `setTimeout`、`clearTimeout`、`setInterval`、`clearInterval`、`setImmediate`、`queueMicrotask`
