/// Format logs properly on Lambda
pub fn tracing_subscriber_fmt() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();
}
