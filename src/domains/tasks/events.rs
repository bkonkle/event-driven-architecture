use chrono::{DateTime, Utc};
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use super::{inputs, Task};

use Event::{Created, Deleted, Updated};

/// Task events
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Event {
    /// A Task was successfully created
    Created {
        /// The ID of the Task that was created
        id: String,

        /// The date this instance was created
        created_at: DateTime<Utc>,

        /// The created Task
        task: Task,
    },

    /// A Task was successfully updated
    Updated {
        /// The ID of the Task that was updated
        id: String,

        /// The date this instance was last updated
        updated_at: DateTime<Utc>,

        /// The update to the Task
        update: inputs::Update,
    },

    /// A Task was successfully deleted
    Deleted {
        /// The ID of the Task that was deleted
        id: String,

        /// When the change occurred
        updated_at: DateTime<Utc>,
    },
}

impl Event {
    /// Return the Aggregate ID
    #[allow(dead_code)]
    pub fn id(&self) -> String {
        match self {
            Created { id, .. } | Updated { id, .. } | Deleted { id, .. } => id.to_string(),
        }
    }
}

impl DomainEvent for Event {
    fn event_type(&self) -> String {
        match self {
            Created { .. } => "Task:Created".to_string(),
            Updated { .. } => "Task:Updated".to_string(),
            Deleted { .. } => "Task:Deleted".to_string(),
        }
    }

    #[allow(clippy::unused_self)]
    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
