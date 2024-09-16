use aws_lambda_events::{
    dynamodb::{Event, EventRecord},
    streams::{DynamoDbBatchItemFailure, DynamoDbEventResponse},
};
use aws_sdk_kinesis::primitives::Blob;
use derive_new::new;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};

use crate::domains::DomainEvent;

/// The Kinesis Publisher
#[derive(Clone, Debug, new)]
pub struct Kinesis {
    client: aws_sdk_kinesis::Client,
}

/// The Event Log Record decoded from the DynamoDB change "new image"
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventLogRecord {
    aggregate_type_and_id: String,
    event_type: String,
    aggregate_id: String,
    aggregate_type: String,
    #[serde(with = "serde_bytes")]
    metadata: Vec<u8>,
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
    event_version: String,
    aggregate_id_sequence: usize,
}

impl TryFrom<EventLogRecord> for DomainEvent {
    type Error = serde_json::Error;

    fn try_from(event: EventLogRecord) -> Result<Self, Self::Error> {
        let payload =
            String::from_utf8(event.payload).expect("Cannot convert the Payload to a String");
        let metadata =
            String::from_utf8(event.metadata).expect("Cannot convert the Metadata to a String");

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
    /// Handle the DynamoDB event and publish the domain event to the Kinesis stream
    pub async fn handle(
        &self,
        event: LambdaEvent<Event>,
    ) -> Result<DynamoDbEventResponse, lambda_runtime::Error> {
        tracing::info!(
            "Processing batch of {} events from DynamoDB",
            event.payload.records.len(),
        );

        let mut batch_item_failures = Vec::new();

        for record in event.payload.records.iter() {
            if record.event_name == "INSERT" {
                let event_id = record.event_id.clone();

                tracing::info!("Handling record id: {}", event_id);

                if let Err(error) = self.handle_record(&record).await {
                    tracing::error!(
                        error = ?error, event_id = event_id,
                        "Failed to process event"
                    );

                    batch_item_failures.push(DynamoDbBatchItemFailure {
                        item_identifier: Some(event_id),
                    });
                };
            } else {
                tracing::info!(
                    "Ignoring event {} for id: {}",
                    record.event_name,
                    record.event_id,
                );
            }
        }

        Ok(DynamoDbEventResponse {
            batch_item_failures,
        })
    }

    async fn handle_record(&self, record: &EventRecord) -> Result<(), lambda_runtime::Error> {
        let stream_name = std::env::var("EVENT_STREAM_NAME").unwrap_or_default();

        let item = &record.change.new_image;
        let event_log: EventLogRecord = serde_dynamo::from_item(item.clone())?;

        tracing::info!(
            "Publishing domain event {} for id {} to {}",
            event_log.event_type,
            event_log.aggregate_id,
            stream_name
        );

        let event: DomainEvent = event_log.clone().try_into()?;
        let data = serde_json::to_string(&event)?;

        self.client
            .put_record()
            .stream_name(stream_name)
            .partition_key(event_log.aggregate_type)
            .data(Blob::new(data))
            .send()
            .await?;

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
