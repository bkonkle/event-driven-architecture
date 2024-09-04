//! # A demo project for a simple CQRS/ES workflow
#![forbid(unsafe_code)]

mod domains;
mod http;
mod utils;

#[macro_use]
extern crate log;

use std::{io, panic::PanicInfo, sync::Arc};

use aws_config::BehaviorVersion;
use axum::{
    routing::{get, post},
    Router,
};
use backtrace::Backtrace;
use cqrs_es::{
    persist::{PersistedEventStore, ViewRepository},
    CqrsFramework,
};
use crossterm::{execute, style::Print};
use domains::tasks::{self, cqrs::init_repo, Task};
use dynamo_es::DynamoEventRepository;
use tower_http::trace;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    tasks_repo: Arc<Box<dyn ViewRepository<tasks::View, Task>>>,
    tasks_cqrs: Arc<CqrsFramework<Task, PersistedEventStore<DynamoEventRepository, Task>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let localstack_endpoint = std::env::var("LOCALSTACK_ENDPOINT").unwrap_or_default();

    let mut config = aws_config::defaults(BehaviorVersion::latest());
    if localstack_endpoint != "" {
        config = config.endpoint_url(&localstack_endpoint);
    }
    let config = config.load().await;

    let client = aws_sdk_dynamodb::Client::new(&config);

    let tasks_repo = init_repo(client.clone());

    let state = AppState {
        tasks_repo: tasks_repo.clone(),
        tasks_cqrs: tasks::cqrs::init(client.clone(), tasks_repo),
    };

    let app = Router::new()
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .route("/tasks", post(http::tasks_create))
        .route(
            "/tasks/:id",
            get(http::tasks_get)
                .patch(http::tasks_update)
                .delete(http::tasks_delete),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!(
        "Started on port: {port}",
        port = listener.local_addr().expect("No server address found")
    );

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// A generic function to log stacktraces on panic
pub fn handle_panic(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        execute!(
            io::stdout(),
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            ))
        )
        .unwrap();
    }
}
