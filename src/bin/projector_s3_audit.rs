use aws_config::BehaviorVersion;
use aws_lambda_events::event::kinesis::KinesisEvent;
use event_driven_architecture::projectors::s3_audit::S3Audit;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    let handler = S3Audit::new(s3_client);

    lambda_runtime::run(service_fn(|event: LambdaEvent<KinesisEvent>| async {
        handler.handle(event).await
    }))
    .await
}
