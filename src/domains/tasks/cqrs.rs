use std::{env, sync::Arc};

use cqrs_es::{
    persist::{PersistedEventStore, ViewRepository},
    CqrsFramework,
};
use dynamo_es::{DynamoEventRepository, DynamoViewRepository};

use super::{Query, Services, Task, View};

/// Initialize the Tasks CqrsFramework
pub fn init(
    client: aws_sdk_dynamodb::Client,
    repo: Arc<Box<dyn ViewRepository<View, Task>>>,
) -> Arc<CqrsFramework<Task, PersistedEventStore<DynamoEventRepository, Task>>> {
    let event_log_table =
        env::var("EVENT_LOG_TABLE_NAME").unwrap_or("event-driven-dev-event-log".to_string());

    let event_snapshots_table = env::var("EVENT_SNAPSHOTS_TABLE_NAME")
        .unwrap_or("event-driven-dev-event-snapshots".to_string());

    let store: PersistedEventStore<DynamoEventRepository, Task> =
        PersistedEventStore::new_snapshot_store(
            DynamoEventRepository::new(client.clone())
                .with_tables(&event_log_table, &event_snapshots_table),
            5,
        );

    let query = Box::new(Query::new(repo));

    Arc::new(CqrsFramework::new(store, vec![query], Services::default()))
}

/// Initialize the Tasks View Repository
pub fn init_repo(client: aws_sdk_dynamodb::Client) -> Arc<Box<dyn ViewRepository<View, Task>>> {
    let tasks_view_table =
        env::var("TASKS_VIEW_TABLE_NAME").unwrap_or("event-driven-dev-tasks-view".to_string());

    Arc::new(Box::new(DynamoViewRepository::new(
        &tasks_view_table,
        client,
    )))
}
