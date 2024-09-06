# Event Driven APIs with AWS Kinesis

## Local Development

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
