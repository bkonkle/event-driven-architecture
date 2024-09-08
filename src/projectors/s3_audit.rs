use std::str::Utf8Error;

use aws_lambda_events::{
    kinesis::{KinesisEvent, KinesisEventRecord},
    streams::{KinesisBatchItemFailure, KinesisEventResponse},
};
use aws_sdk_s3::{error::SdkError, operation::put_object::PutObjectError, primitives::ByteStream};
use derive_new::new;
use lambda_runtime::LambdaEvent;

use crate::{
    domains::{tasks, DomainEvent},
    utils,
};

#[derive(Clone, Debug, new)]
pub struct S3Audit {
    client: aws_sdk_s3::Client,
}

impl S3Audit {
    pub async fn handle(
        &self,
        event: LambdaEvent<KinesisEvent>,
    ) -> Result<KinesisEventResponse, lambda_runtime::Error> {
        tracing::info!(
            "Processing batch of {} events from Kinesis",
            event.payload.records.len(),
        );

        let mut iter = event.payload.records.iter();
        while let Some(record) = iter.next() {
            let sequence_number = record.kinesis.sequence_number.clone();

            tracing::info!(
                sequence_number = sequence_number,
                "Handling record id: {:?}",
                record.event_id
            );

            if let Err(error) = self.handle_record(record).await {
                tracing::error!(
                    error = ?error, sequence_number = sequence_number,
                    "Failed to process event"
                );

                let error = KinesisBatchItemFailure {
                    item_identifier: sequence_number,
                };

                // To preserve order, fail all remaining items in the batch in addition to this one
                let batch_item_failures: Vec<_> = std::iter::once(error)
                    .chain(iter.by_ref().cloned().map(|rec| KinesisBatchItemFailure {
                        item_identifier: rec.kinesis.sequence_number,
                    }))
                    .collect();

                return Ok(KinesisEventResponse {
                    batch_item_failures,
                });
            };
        }

        Ok(KinesisEventResponse {
            batch_item_failures: vec![],
        })
    }

    async fn handle_record(&self, record: &KinesisEventRecord) -> Result<(), Error> {
        let bucket_name = std::env::var("AUDIT_BUCKET_NAME").unwrap_or_default();

        let record_data = std::str::from_utf8(&record.kinesis.data)
            .map_err(Error::Utf8)?
            .to_string();
        let event: DomainEvent = serde_json::from_str(&record_data).map_err(Error::Json)?;

        println!(">- event -> {:?}", event);

        if event.entity == tasks::AGGREGATE_TYPE {
            let payload: tasks::Event =
                serde_json::from_str(&event.payload).map_err(Error::Json)?;
            if let tasks::Event::Updated { update, .. } = payload {
                if let utils::Update::Value(summary) = update.summary {
                    if summary == "5" {
                        return Err(Error::InvalidSummary(summary));
                    }
                }
            }
        }

        self.client
            .put_object()
            .bucket(bucket_name)
            .key(format!(
                "events/{}/{}-{}.json",
                event.entity, event.id, event.sequence
            ))
            .body(ByteStream::from(record_data.into_bytes()))
            .send()
            .await
            .map_err(Error::S3PutError)?;

        Ok(())
    }
}

/// S3 Audit errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Utf8 conversion error: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("JSON conversion error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid Summary error (used to demonstrate retries)
    #[error("Invalid summary: {0}")]
    InvalidSummary(String),

    #[error("S3 Put Object error: {0}")]
    S3PutError(#[from] SdkError<PutObjectError>),
}
