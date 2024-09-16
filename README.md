# Event Driven APIs with AWS Kinesis

AWS provides a strong set of tools for many different server-side event-driven approaches, bringing diverse programming language ecosystems and application platforms together with common infrastructure. [Kinesis](https://aws.amazon.com/kinesis/) is a data streaming service that provides a good balance of relative simplicity with powerful scaling. It’s able to handle global-scale workloads with consistent performance and reliability, and it doesn’t take an expert to configure and manage.

A strategy that I’ve found very effective on recent projects is to use a [DynamoDB](https://aws.amazon.com/dynamodb/) table as an immutable event log, streaming events to a serverless “publisher” Lambda function that formats the changes as domain events and broadcasts them over a Kinesis stream for many different listeners. These listeners can be both internal to the project and external within other teams. Since AWS offers a very effective multi-region replication feature with DynamoDB’s global tables, it forms an excellent backbone to keep multiple regions in sync and allow for smooth automatic failover in the event a particular region goes down.

![Overview Diagram](https://github.com/bkonkle/event-driven-architecture/blob/main/docs/overview-diagram.png?raw=true)

This project is a basic demonstration of this architecture, while omitting the multi-region configuration for simplicity.

## Local Development

Since API Gateway is a premium LocalStack feature, my goal is to run locally without it using an [Axum](https://github.com/tokio-rs/axum) server process. It will connect to LocalStack for DynamoDB and the downstream Kinesis and Lambda operations that it will trigger, however.

To get started, first make sure you have `direnv`, `docker-compose`, `pipx` and `cargo-make` installed. Use them to set up a simple development environment with the following commands:

```sh
pipx install cargo-lambda

cp .envrc.example .envrc

direnv allow

cargo make lambda-build

cargo make docker up -d

cargo make tf init

cargo make tf apply --auto-approve

cargo make dev
```

The Lambda functions are output to `target/lambda/`, where Terraform looks for packaging them.

The API process runs independently locally, but will be hosted via API Gateway when deployed.

## Manual Testing

To test, start off by creating a new Task by calling `POST http://localhost:3000/tasks`:

```json
{
    "name": "My New Task",
    "summary": "Task Summary"
}
```

Retrieve the task to make sure it was saved correctly by calling `GET /path/to/api/gateway/dev/tasks/{id}`. You should see the task you created:

```json
{
    "aggregate_type": "Task",
    "command_id": "01J73SBWHEBMP8Z07HYZR5BER4",
    "id": "01J73SBWHE373VXWZTF7SJADD9",
    "task": {
        "id": "01J73SBWHE373VXWZTF7SJADD9",
        "created_at": "2024-09-06T13:46:18.567497226Z",
        "updated_at": "2024-09-06T13:46:18.567497226Z",
        "name": "My New Task",
        "summary": "My task summary",
        "done": false,
        "deleted": false
    }
}
```

In this example app, the IDs are lexicographically sortable, so you can compare Command IDs with each other to determine if the last command executed is older or newer than another Command ID.

To update the task, call `PATCH /path/to/api/gateway/dev/tasks/{id}`:

```json
{
    "summary": "My new task summary"
}
```

You should see the updated record returned in the response.

## Deployment

First, create an AWS user for your project root. If you call your project namespace "event-driven", then your user would be "event-driven-root". This should not be a login user, but should have CLI access for Terraform. It should have a set of permissions similar to the policy json in `infra/aws/bootstrap/event-driven-root-access.json`.

Generate an access token, and save the credentials to `~/.aws/credentials` with a profile name of `event-driven-root`.

Now you're ready to bootstrap your Terraform state. Run the following to set up a new S3 bucket and DynamoDB table for your Terraform state:

```sh
cd infra/aws/bootstrap

AWS_PROFILE=event-driven-root terraform init

AWS_PROFILE=event-driven-root terraform apply
```

Now you can deploy your initial dev environment:

```sh
cd infra/aws

cp dev.s3.tfbackend.example dev.s3.tfbackend

AWS_PROFILE=event-driven-root terraform init -backend-config=dev.s3.tfbackend

cd ../../

AWS_PROFILE=event-driven-root cargo make tf-dev apply
```

From there, you can find your API Gateway in the AWS Console. Use the URL for the "dev" stage to test your API. Your default API Gateway will be prefixed with the `/dev` stage token, but you can remove this with url rewriting when you set up a load-balancing proxy.
