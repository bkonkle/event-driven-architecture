use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct DomainEvent {
    pub id: String,
    pub entity: String,
    pub sequence: usize,
    pub event_type: String,
    pub event_version: String,
    pub payload: Value,
    pub metadata: Value,
}
