use aws_lambda_events::kinesis::KinesisEvent;
use aws_sdk_s3::primitives::ByteStream;
use derive_new::new;
use lambda_runtime::LambdaEvent;

use crate::domains::DomainEvent;

const BUCKET_NAME: &str = "event-audit";

#[derive(Clone, Debug, new)]
pub struct S3Audit {
    client: aws_sdk_s3::Client,
}

impl S3Audit {
    pub async fn handle(
        &self,
        event: LambdaEvent<KinesisEvent>,
    ) -> Result<(), lambda_runtime::Error> {
        tracing::info!(
            "Processing batch of {} events from Kinesis",
            event.payload.records.len(),
        );

        for record in event.payload.records {
            let record_data = std::str::from_utf8(&record.kinesis.data)?.to_string();
            let event: DomainEvent = serde_json::from_str(&record_data)?;

            println!(">- event -> {:?}", event);

            self.client
                .put_object()
                .bucket(BUCKET_NAME)
                .key(format!(
                    "events/{}/{}-{}.json",
                    event.entity, event.id, event.sequence
                ))
                .body(ByteStream::from(record_data.into_bytes()))
                .send()
                .await?;
        }

        Ok(())
    }
}
