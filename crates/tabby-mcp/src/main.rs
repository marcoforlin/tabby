use std::sync::Arc;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};
mod common;
use common::counter::Counter;
use tabby_index_cli::get_repos;
// use tabby_db::DbConn;
// use tabby_common::path::tabby_root;
const BIND_ADDRESS: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> anyhow::Result<()> {


    // let path = tabby_root().join("db.sqlite");
    // let db = DbConn::new(path.as_path())
    //     .await
    //     .expect("Must be able to initialize db");
    // db.finalize_stale_job_runs()
    //     .await
    //     .expect("Must be able to finalize stale job runs");
    //
    // // let logger2 = create_event_logger(db.clone());
    // // Ensure query does not break on the join
    // let repos = db.list_provided_repositories(vec![], Some("github".into()), None, None, None, false)
    //     .await
    //     .unwrap()
    //     .iter()
    //     .map(|repo| repo.name.to_string())
    //     .collect::<Vec<_>>()
    //     .join(", ");
    // println!("repos: {repos:?}");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = SseServerConfig {
        bind: BIND_ADDRESS.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };
    println!("starting...");
    tracing::info!("starting the MCP server");

    tracing::info!("available repos:{0}",get_repos().await.unwrap());

    let (sse_server, router) = SseServer::new(config);

    // Do something with the router, e.g., add routes or middleware

    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;

    let ct = sse_server.config.ct.child_token();

    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        ct.cancelled().await;
        tracing::info!("sse server cancelled");
    });

    tokio::spawn(async move {
        if let Err(e) = server.await {
            tracing::error!(error = %e, "sse server shutdown with error");
        }
    });
    println!("listening...");
    let ct = sse_server.with_service(common::codesearch_service::CodeSearchService::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}