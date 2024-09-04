# Event Driven APIs with AWS Kinesis

***(WIP)***

```sh
cargo make tf init

cargo make tf apply --auto-approve

pipx install cargo-lambda

cargo lambda build --bin publisher_kinesis
```

This outputs to `target/lambda/publisher_kinesis/`.
