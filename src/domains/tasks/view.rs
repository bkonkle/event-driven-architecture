use std::sync::Arc;

use async_trait::async_trait;
use cqrs_es::{
    persist::{PersistenceError, ViewContext, ViewRepository},
    Aggregate, EventEnvelope, View as CqrsView,
};
use serde::{Deserialize, Serialize};

use super::{Task, AGGREGATE_TYPE};

/// The default View for a Task
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct View {
    /// The Aggregage type, to differentiate the many types stored in the default view
    pub aggregate_type: String,

    /// The last Command ID processed
    pub command_id: String,

    /// The Task id
    pub id: String,

    /// The primary entity, a Task
    pub task: Task,
}

impl cqrs_es::View<Task> for View {
    fn update(&mut self, event: &EventEnvelope<Task>) {
        self.id.clone_from(&event.aggregate_id);
        self.aggregate_type = AGGREGATE_TYPE.to_string();

        self.command_id = event
            .metadata
            .get("command_id")
            .unwrap_or(&"".to_string())
            .to_string();

        self.task.apply(event.payload.clone());
    }
}

/// A Query to update Task views in response to Task events
///
/// Note: GenericQuery requires the ViewRepository to be Sized, not dynamic. The only reason this
/// is custom rather than generic is to avoid that requirement.
pub struct Query {
    tasks: Arc<Box<dyn ViewRepository<View, Task>>>,
}

impl Query {
    /// Create a new instance
    pub fn new(tasks: Arc<Box<dyn ViewRepository<View, Task>>>) -> Self {
        Self { tasks }
    }

    async fn update(
        &self,
        task_id: &str,
        events: &[EventEnvelope<Task>],
    ) -> Result<(), PersistenceError> {
        let (mut view, view_context) = match self.tasks.load_with_context(task_id).await? {
            None => {
                let view_context = ViewContext::new(task_id.to_string(), 0);
                (Default::default(), view_context)
            }
            Some((view, context)) => (view, context),
        };

        for event in events {
            let command_id = event
                .metadata
                .get("command_id")
                .unwrap_or(&"".to_string())
                .to_string();

            view.id = task_id.to_string();
            view.aggregate_type = AGGREGATE_TYPE.to_string();
            view.command_id.clone_from(&command_id);

            view.update(event);
        }

        self.tasks.update_view(view, view_context).await
    }
}

#[async_trait]
impl cqrs_es::Query<Task> for Query {
    async fn dispatch(&self, task_id: &str, events: &[EventEnvelope<Task>]) {
        match self.update(task_id, events).await {
            Ok(_) => {}
            Err(err) => {
                error!(
                    err:err = err,
                    task_id = task_id;
                    "TaskQuery: {}",
                    err,
                );
            }
        }
    }
}
