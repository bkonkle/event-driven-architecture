use derive_new::new;
use serde::{Deserialize, Serialize};

/// Domain events formatted in a cosistent way so that they can be shared across teams
#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct DomainEvent {
    /// The Aggregate ID
    pub id: String,

    /// The Aggregate type
    pub entity: String,

    /// The event sequence number
    pub sequence: usize,

    /// The event type
    pub event_type: String,

    /// The event version
    pub event_version: String,

    /// The event payload
    pub payload: String,

    /// The event metadata
    pub metadata: String,
}
