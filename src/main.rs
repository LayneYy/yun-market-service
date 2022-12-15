use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    middleware::from_extractor,
    Router,
    routing::get,
};
use tracing_appender::{non_blocking, rolling};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, registry};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use yun_market_service::{
    context::{CTX, config::LogConf},
    controller::bank_issue_query::bank_issue_query,
    middleware,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let log_config = &CTX.config.log;
    let _g = init_log(log_config);

    let app = Router::new()
        .route("/", get(bank_issue_query))
        .layer(from_extractor::<middleware::IpWhiteList>());

    let server_config = &CTX.config.server;
    let addr = format!("{}:{}", server_config.addr, server_config.port);

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

fn init_log(log_config: &LogConf) -> Option<WorkerGuard> {
    let env_filter = || EnvFilter::from_default_env();
    match (log_config.console_log, &log_config.file_log) {
        (console, Some(file_log_conf)) if file_log_conf.enable => {
            let file_appender = rolling::daily(&file_log_conf.log_path, &file_log_conf.log_prefix);
            let (non_blocking, g) = non_blocking(file_appender);
            let file_log = fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_filter(env_filter());
            let registry = registry().with(file_log);
            if console {
                let console_log = fmt::layer().with_filter(env_filter());
                registry.with(console_log).init();
            }
            Some(g)
        }
        (true, _) => {
            let console_log = fmt::layer().with_filter(env_filter());
            registry().with(console_log).init();
            None
        }
        _ => None,
    }
}