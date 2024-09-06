# Event Driven APIs with AWS Kinesis

## Local Development

This is a work-in-progress. Since API Gateway is a premium LocalStack feature, my goal is to run locally without it using a local process. It will connect to LocalStack for DynamoDB and the downstream Kinesis and Lambda operations that it will trigger, however.

TODO: Need to update local operation after adapting for API Gateway.

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

The Lambda functions are output to `target/lambda/` and are ready for packaging by Terraform.

The API process runs locally, and will be mounted on API Gateway when deployed.

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

From there, you can find your API Gateway in the AWS Console. Use the URL for the "dev" stage to test your API. Start off by creating a new Task by calling `POST /path/to/api/gateway/dev/tasks`:

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
