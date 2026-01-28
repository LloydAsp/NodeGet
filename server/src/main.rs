#![feature(duration_millis_float)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::similar_names,
    clippy::too_many_lines,
    dead_code
)]

use crate::db_connection::init_db_connection;
use crate::rpc::agent::RpcServer as AgentRpcServer;
use crate::rpc::nodeget::RpcServer as NodegetRpcServer;
use crate::rpc::task::{RpcServer, TaskManager};
use axum::Router;
use axum::routing::any;
use jsonrpsee::server::{Server, stop_channel};
use log::{Level, info};
use nodeget_lib::config::server::ServerConfig;
use nodeget_lib::utils::compare_uuid;
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::OnceLock;
#[cfg(all(not(target_os = "windows"), feature = "jemalloc"))]
use tikv_jemallocator::Jemalloc;
use tokio::sync::OnceCell;
use tower::Service;
#[cfg(all(not(target_os = "windows"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod db_connection;
mod entity;
mod rpc;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();

#[tokio::main]
async fn main() {
    println!("Starting nodeget-server");

    // Config Parse
    let config = ServerConfig::get_and_parse_config("./config.toml")
        .await
        .unwrap();

    // Log init
    simple_logger::init_with_level(Level::from_str(&config.log_level).unwrap()).unwrap();

    // Jemalloc Mem Debug
    #[cfg(all(not(target_os = "windows"), feature = "jemalloc"))]
    tokio::spawn(async {
        loop {
            use tikv_jemalloc_ctl::{epoch, stats};
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            if epoch::advance().is_err() {
                return;
            }

            let allocated = stats::allocated::read().unwrap();
            let active = stats::active::read().unwrap();
            let resident = stats::resident::read().unwrap();
            let mapped = stats::mapped::read().unwrap();

            info!(
                "MEM STATS (Jemalloc Only): App Logic: {:.2} MB | Allocator Active: {:.2} MB | RSS (Resident): {:.2} MB | Mapped: {:.2} MB",
                allocated as f64 / 1024.0 / 1024.0,
                active as f64 / 1024.0 / 1024.0,
                resident as f64 / 1024.0 / 1024.0,
                mapped as f64 / 1024.0 / 1024.0
            );
        }
    });

    // 对比 Uuid，发送警告
    let _ = compare_uuid(config.server_uuid);

    info!("Starting nodeget-server with config: {config:?}");

    // 初始化全局 Config
    SERVER_CONFIG.set(config.clone()).unwrap();

    // 连接数据库
    init_db_connection().await;

    let task_manager = TaskManager::new();
    let mut rpc_module = rpc::nodeget::NodegetServerRpcImpl.into_rpc();
    rpc_module
        .merge(rpc::agent::AgentRpcImpl.into_rpc())
        .unwrap();
    rpc_module
        .merge(
            rpc::task::TaskRpcImpl {
                manager: task_manager.clone(),
            }
            .into_rpc(),
        )
        .unwrap();

    let (stop_handle, _server_handle) = stop_channel();

    let jsonrpc_service = Server::builder()
        .set_config(
            jsonrpsee::server::ServerConfig::builder()
                .max_response_body_size(u32::MAX)
                .max_request_body_size(u32::MAX)
                .build(),
        )
        .to_service_builder()
        .build(rpc_module, stop_handle);

    let app = Router::new().fallback(any(move |req: axum::extract::Request| {
        let mut rpc_service = jsonrpc_service.clone();
        async move { rpc_service.call(req).await.unwrap() }
    }));

    let listener = tokio::net::TcpListener::bind(config.ws_listener.parse::<SocketAddr>().unwrap())
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
