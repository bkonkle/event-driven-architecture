use aws_lambda_events::kinesis::KinesisEvent;
use lambda_runtime::LambdaEvent;

pub async fn handle(event: LambdaEvent<KinesisEvent>) -> Result<(), lambda_runtime::Error> {
    tracing::info!(
        "Processing batch of {} events from Kinesis: {:?}",
        event.payload.records.len(),
        event,
    );

    Ok(())
}
