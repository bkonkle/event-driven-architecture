use aws_lambda_events::event::kinesis::KinesisEvent;
use event_driven_architecture::projectors::s3_audit::handle;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    lambda_runtime::run(service_fn(|event: LambdaEvent<KinesisEvent>| async {
        handle(event).await
    }))
    .await
}
