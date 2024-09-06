use aws_config::BehaviorVersion;
use aws_lambda_events::event::dynamodb::Event;
use event_driven_architecture::{publishers, utils::lambda};
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::tracing_subscriber_fmt();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let kinesis_client = aws_sdk_kinesis::Client::new(&config);
    let handler = publishers::Kinesis::new(kinesis_client);

    lambda_runtime::run(service_fn(|event: LambdaEvent<Event>| async {
        handler.handle(event).await
    }))
    .await
}
