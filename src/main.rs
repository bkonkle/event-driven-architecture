//! A demo project for a simple CQRS/ES workflow
#![forbid(unsafe_code)]

mod domains;
mod http;
mod utils;

#[macro_use]
extern crate log;

use std::{io, panic::PanicInfo, sync::Arc};

use anyhow::anyhow;
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
use utils::lambda;

#[derive(Clone)]
struct AppState {
    tasks_repo: Arc<Box<dyn ViewRepository<tasks::View, Task>>>,
    tasks_cqrs: Arc<CqrsFramework<Task, PersistedEventStore<DynamoEventRepository, Task>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if is_running_on_lambda() {
        lambda::tracing_subscriber_fmt();
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    let localstack_endpoint = std::env::var("LOCALSTACK_ENDPOINT").unwrap_or_default();
    let environment = std::env::var("ENV").unwrap_or_default();

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

    let env_path = if environment == "local" {
        "".to_string()
    } else {
        format!("/{}", environment)
    };

    let app = Router::new()
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .nest(
            &env_path,
            Router::new()
                .route("/tasks", post(http::tasks_create))
                .route(
                    "/tasks/:id",
                    get(http::tasks_get)
                        .patch(http::tasks_update)
                        .delete(http::tasks_delete),
                )
                .with_state(state),
        );

    if is_running_on_lambda() {
        // To run with AWS Lambda runtime, wrap in our `LambdaLayer`
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        if let Err(error) = lambda_http::run(app).await {
            return Err(anyhow!("Lambda HTTP error: {}", error));
        };
    } else {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .expect("Unable to bind TcpListener");

        info!(
            "Started on port: {port}",
            port = listener.local_addr().expect("No server address found")
        );

        axum::serve(listener, app).await?;
    }

    Ok(())
}

fn is_running_on_lambda() -> bool {
    std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok()
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
