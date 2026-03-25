use rquickjs::{AsyncContext, AsyncRuntime, Error};
use rquickjs::prelude::{Async, Func};

pub mod nodeget;

pub fn js_error(stage: &'static str, message: impl ToString) -> Error {
    Error::new_from_js_message(stage, "String", message.to_string())
}

pub async fn js_runner(js_code: impl ToString) -> Result<String, Error> {
    let rt = AsyncRuntime::new()?;
    let ctx = AsyncContext::full(&rt).await?;

    ctx.with(|ctx| {
        let global = ctx.globals();

        global.set("nodeget", Func::from(Async(nodeget::js_nodeget)))?;

        ctx.eval::<(), _>(
            r#""#,
        )
    }).await?;

    rt.idle().await;

    todo!()
}