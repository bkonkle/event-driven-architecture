use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct DomainEvent {
    pub id: String,
    pub entity: String,
    pub sequence: usize,
    pub event_type: String,
    pub event_version: String,
    pub payload: String,
    pub metadata: String,
}
