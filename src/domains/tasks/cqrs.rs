use std::sync::Arc;

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
    let store: PersistedEventStore<DynamoEventRepository, Task> =
        PersistedEventStore::new_snapshot_store(
            DynamoEventRepository::new(client.clone()).with_tables("event_log", "event_snapshots"),
            5,
        );

    let query = Box::new(Query::new(repo));

    Arc::new(CqrsFramework::new(store, vec![query], Services::default()))
}

pub fn init_repo(client: aws_sdk_dynamodb::Client) -> Arc<Box<dyn ViewRepository<View, Task>>> {
    Arc::new(Box::new(DynamoViewRepository::new("tasks_view", client)))
}
