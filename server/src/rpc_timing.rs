use jsonrpsee::server::middleware::rpc::{Batch, Notification, Request, RpcServiceT};
use log::{Level, debug, error, info, trace, warn};
use std::future::Future;
use std::time::Instant;

#[derive(Clone)]
pub struct RpcTimingMiddleware<S> {
    pub service: S,
    pub level: Level,
}

pub fn parse_rpc_timing_log_level(raw: Option<&str>) -> (Level, Option<String>) {
    let Some(level_str) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return (Level::Trace, None);
    };

    if level_str.eq_ignore_ascii_case("error") {
        (Level::Error, None)
    } else if level_str.eq_ignore_ascii_case("warn") {
        (Level::Warn, None)
    } else if level_str.eq_ignore_ascii_case("info") {
        (Level::Info, None)
    } else if level_str.eq_ignore_ascii_case("debug") {
        (Level::Debug, None)
    } else if level_str.eq_ignore_ascii_case("trace") {
        (Level::Trace, None)
    } else {
        (Level::Trace, Some(level_str.to_owned()))
    }
}

fn log_with_level(level: Level, message: &str) {
    match level {
        Level::Error => error!("{message}"),
        Level::Warn => warn!("{message}"),
        Level::Info => info!("{message}"),
        Level::Debug => debug!("{message}"),
        Level::Trace => trace!("{message}"),
    }
}

impl<S> RpcServiceT for RpcTimingMiddleware<S>
where
    S: RpcServiceT + Send + Sync + Clone + 'static,
{
    type MethodResponse = S::MethodResponse;
    type NotificationResponse = S::NotificationResponse;
    type BatchResponse = S::BatchResponse;

    fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Self::MethodResponse> + Send + 'a {
        let method_name = request.method_name().to_owned();
        let request_id = format!("{:?}", request.id());
        let level = self.level;
        let service = self.service.clone();
        let started_at = Instant::now();

        async move {
            let response = service.call(request).await;
            let elapsed_us = started_at.elapsed().as_micros();
            log_with_level(
                level,
                &format!(
                    "rpc.call completed method={method_name} id={request_id} elapsed_us={elapsed_us}"
                ),
            );
            response
        }
    }

    fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Self::BatchResponse> + Send + 'a {
        let batch_size = batch.len();
        let mut method_names = Vec::with_capacity(batch_size);
        for entry in batch.iter() {
            match entry {
                Ok(item) => method_names.push(item.method_name().to_owned()),
                Err(_) => method_names.push("<invalid>".to_owned()),
            }
        }
        let methods = if method_names.is_empty() {
            "<empty>".to_owned()
        } else {
            method_names.join(",")
        };

        let level = self.level;
        let service = self.service.clone();
        let started_at = Instant::now();

        async move {
            let response = service.batch(batch).await;
            let elapsed_us = started_at.elapsed().as_micros();
            log_with_level(
                level,
                &format!(
                    "rpc.batch completed size={batch_size} methods={methods} elapsed_us={elapsed_us}"
                ),
            );
            response
        }
    }

    fn notification<'a>(&self, n: Notification<'a>) -> impl Future<Output = Self::NotificationResponse> + Send + 'a {
        let method_name = n.method_name().to_owned();
        let level = self.level;
        let service = self.service.clone();
        let started_at = Instant::now();

        async move {
            let response = service.notification(n).await;
            let elapsed_us = started_at.elapsed().as_micros();
            log_with_level(
                level,
                &format!("rpc.notification completed method={method_name} elapsed_us={elapsed_us}"),
            );
            response
        }
    }
}
