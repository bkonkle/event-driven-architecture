use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct DomainEvent {
    id: String,
    entity: String,
    sequence: usize,
    event_type: String,
    event_version: String,
    payload: Value,
    metadata: Value,
}
