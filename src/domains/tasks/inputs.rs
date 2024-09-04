use serde::{Deserialize, Serialize};

use crate::utils;

use super::Task;

/// An input type for Task creation
#[derive(Clone, Debug, Default, Eq, Serialize, Deserialize, PartialEq)]
pub struct Create {
    /// A name
    pub name: String,

    /// An optional summary
    pub summary: Option<String>,
}

impl From<Task> for Create {
    fn from(task: Task) -> Self {
        Create {
            name: task.name.clone(),
            summary: task.summary.clone(),
        }
    }
}

/// An input type that supports partial Task updates
#[derive(Clone, Debug, Default, Eq, Serialize, Deserialize, PartialEq)]
pub struct Update {
    /// A name
    pub name: Option<String>,

    /// An optional summary
    pub summary: utils::Update<String>,

    /// Whether this Task is completed or not
    pub done: Option<bool>,
}
