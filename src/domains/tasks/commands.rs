use serde::{Deserialize, Serialize};

use super::inputs;

/// Task Aggregate Commands
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Command {
    /// Create a new Task
    Create { id: String, input: inputs::Create },

    /// Update an existing Task
    Update(inputs::Update),

    /// Remove an existing Task
    Delete,
}
