use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{
    domains,
    utils::Update::{Empty, Unchanged, Value},
};

use super::{Command, Event};

use Command::{Create, Delete, Update};
use Event::{Created, Deleted, Updated};

/// A Task as aggregated within the Event Store
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct Task {
    /// A unique ID
    pub id: String,

    /// The created date
    pub created_at: DateTime<Utc>,

    /// The last updated date
    pub updated_at: DateTime<Utc>,

    /// A name
    pub name: String,

    /// An optional summary
    pub summary: Option<String>,

    /// Whether this Task is completed or not
    pub done: bool,

    /// Whether this Task is is active or has been removed
    pub deleted: bool,
}

/// The Aggregate Type constant
pub const AGGREGATE_TYPE: &str = "Task";

/// Services needed by the Task Aggregate (currently none)
#[derive(Clone, Default)]
pub struct Services {}

#[async_trait]
impl Aggregate for Task {
    type Command = Command;
    type Event = Event;
    type Error = domains::Error;
    type Services = Services;

    fn aggregate_type() -> String {
        AGGREGATE_TYPE.to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            Create { id, input } => {
                self.validate_new()?;

                let created_at = Utc::now();

                Ok(vec![Created {
                    id: id.clone(),
                    created_at,
                    task: Task {
                        id,
                        created_at,
                        updated_at: created_at,
                        name: input.name,
                        summary: input.summary,
                        done: false,
                        deleted: false,
                    },
                }])
            }

            Update(update) => {
                self.validate_existing()?;

                Ok(vec![Updated {
                    id: self.id.clone(),
                    updated_at: Utc::now(),
                    update,
                }])
            }

            Delete => {
                self.validate_existing()?;

                Ok(vec![Deleted {
                    id: self.id.clone(),
                    updated_at: Utc::now(),
                }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Created {
                id,
                task,
                created_at,
            } => {
                self.id = id;
                self.created_at = created_at;
                self.updated_at = created_at;
                self.name = task.name;
                self.summary = task.summary;
                self.done = task.done;
                self.deleted = task.deleted;
            }

            Updated {
                update, updated_at, ..
            } => {
                if let Some(name) = update.name {
                    self.name = name;
                }

                match update.summary {
                    Unchanged => {
                        // Leave unchanged
                    }
                    Empty => {
                        self.summary = None;
                    }
                    Value(summary) => {
                        self.summary = Some(summary);
                    }
                }

                if let Some(done) = update.done {
                    self.done = done;
                }

                self.updated_at = updated_at;
            }

            Deleted { updated_at, .. } => {
                self.deleted = true;
                self.updated_at = updated_at;
            }
        }
    }
}

impl Task {
    fn validate_new(&self) -> Result<(), domains::Error> {
        if !self.id.is_empty() {
            // A Universe with this ID already exists, so there is a uniqueness conflict
            return Err(domains::Error::Uniqueness {
                field: "id".to_string(),
            });
        }

        Ok(())
    }

    fn validate_existing(&self) -> Result<(), domains::Error> {
        if self.id.is_empty() {
            return Err(domains::Error::NotFound {
                entity: AGGREGATE_TYPE.to_string(),
            });
        }

        if self.deleted {
            // This Universe has already been deleted, so this action is Forbidden
            return Err(domains::Error::Forbidden);
        }

        Ok(())
    }
}
