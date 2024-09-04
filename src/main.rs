//! # A demo project for a simple CQRS/ES workflow
#![forbid(unsafe_code)]

mod domains;
mod http;
mod utils;

#[macro_use]
extern crate log;

use std::sync::Arc;

use aws_config::BehaviorVersion;
use axum::{
    routing::{get, post},
    Router,
};
use cqrs_es::{
    persist::{PersistedEventStore, ViewRepository},
    CqrsFramework,
};
use domains::tasks::{self, cqrs::init_repo, Task};
use dynamo_es::DynamoEventRepository;

#[derive(Clone)]
struct AppState {
    tasks_repo: Arc<Box<dyn ViewRepository<tasks::View, Task>>>,
    tasks_cqrs: Arc<CqrsFramework<Task, PersistedEventStore<DynamoEventRepository, Task>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
