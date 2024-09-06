use aws_config::BehaviorVersion;
use aws_lambda_events::event::dynamodb::Event;
use event_driven_architecture::publishers;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let kinesis_client = aws_sdk_kinesis::Client::new(&config);
    let handler = publishers::Kinesis::new(kinesis_client);

    lambda_runtime::run(service_fn(|event: LambdaEvent<Event>| async {
        handler.handle(event).await
    }))
    .await
}
