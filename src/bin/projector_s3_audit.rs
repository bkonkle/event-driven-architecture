use aws_config::BehaviorVersion;
use aws_lambda_events::event::kinesis::KinesisEvent;
use event_driven_architecture::{projectors::s3_audit::S3Audit, utils::lambda};
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::tracing_subscriber_fmt();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    let handler = S3Audit::new(s3_client);

    lambda_runtime::run(service_fn(|event: LambdaEvent<KinesisEvent>| async {
        handler.handle(event).await
    }))
    .await
}
