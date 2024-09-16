use serde::{Deserialize, Serialize};

use super::inputs;

/// Task Aggregate Commands
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Command {
    /// Create a new Task
    Create {
        /// The Task ID to create (auto-generated in the http Create handler)
        id: String,

        /// The Create input
        input: inputs::Create,
    },

    /// Update an existing Task
    Update(inputs::Update),

    /// Remove an existing Task
    Delete,
}
