use aws_lambda_events::dynamodb::{Event, EventRecord};
use aws_sdk_kinesis::primitives::Blob;
use derive_new::new;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};

use crate::domains::DomainEvent;

const STREAM_NAME: &str = "event-stream";

/// The Kinesis Publisher
#[derive(Clone, Debug, new)]
pub struct Kinesis {
    client: aws_sdk_kinesis::Client,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventLogRecord {
    aggregate_type_and_id: String,
    event_type: String,
    aggregate_id: String,
    aggregate_type: String,
    metadata: String,
    payload: String,
    event_version: String,
    aggregate_id_sequence: usize,
}

impl TryFrom<EventLogRecord> for DomainEvent {
    type Error = serde_json::Error;

    fn try_from(event: EventLogRecord) -> Result<Self, Self::Error> {
        let payload = serde_json::to_value(&event.payload)?;
        let metadata = serde_json::to_value(&event.metadata)?;

        Ok(DomainEvent::new(
            event.aggregate_id,
            event.aggregate_type,
            event.aggregate_id_sequence,
            event.event_type,
            event.event_version,
            payload,
            metadata,
        ))
    }
}

impl Kinesis {
    pub async fn handle(&self, event: LambdaEvent<Event>) -> Result<(), lambda_runtime::Error> {
        tracing::info!(
            "Processing batch of {} events from DynamoDB",
            event.payload.records.len(),
        );

        for record in event.payload.records {
            if record.event_name == "INSERT" {
                tracing::info!("Handling record id: {}", record.event_id);

                self.handle_record(&record).await?;
            } else {
                tracing::info!(
                    "Ignoring event {} for id: {}",
                    record.event_name,
                    record.event_id,
                );
            }
        }

        Ok(())
    }

    pub async fn handle_record(&self, record: &EventRecord) -> Result<(), lambda_runtime::Error> {
        let item = &record.change.new_image;

        let event_log: EventLogRecord = serde_dynamo::from_item(item.clone())?;

        tracing::info!(
            "Publishing domain event {} for id {} to {}",
            event_log.event_type,
            event_log.aggregate_id,
            STREAM_NAME
        );

        let event: DomainEvent = event_log.clone().try_into()?;
        let data = serde_json::to_string(&event)?;

        self.client
            .put_record()
            .stream_name(STREAM_NAME)
            .partition_key(event_log.aggregate_type)
            .data(Blob::new(data));

        Ok(())
    }
}

/// Publisher errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Not found error
    #[error("{attribute} not found")]
    Invalid {
        /// The attribute that was not found
        attribute: String,
    },
}
