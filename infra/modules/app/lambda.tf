module "label_publisher_kinesis" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "publisher-kinesis"
  tags      = local.common_tags
  delimiter = "-"
}

module "lambda_publisher_kinesis" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = module.label_publisher_kinesis.id
  description   = "The Kinesis domain event Publisher"
  handler       = "bootstrap"
  runtime       = "provided.al2023"

  source_path = "../../target/lambda/publisher_kinesis"
}

resource "aws_lambda_event_source_mapping" "publisher_kinesis_dynamodb_trigger" {
  event_source_arn       = module.dynamodb_event_log.dynamodb_table_stream_arn
  function_name          = module.lambda_publisher_kinesis.lambda_function_arn
  starting_position      = "LATEST"
  maximum_retry_attempts = 5

  destination_config {
    on_failure {
      destination_arn = module.sqs_publisher_kinesis_dead_letter.queue_arn
    }
  }
}

module "label_projector_s3_audit" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "projector-s3-audit"
  tags      = local.common_tags
  delimiter = "-"
}

module "lambda_projector_s3_audit" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = module.label_projector_s3_audit.id
  description   = "The S3 audit Projector"
  handler       = "bootstrap"
  runtime       = "provided.al2023"

  source_path = "../../target/lambda/projector_s3_audit"
}

resource "aws_lambda_event_source_mapping" "projector_s3_audit_kinesis_trigger" {
  event_source_arn        = resource.aws_kinesis_stream.event_stream.arn
  function_name           = module.lambda_projector_s3_audit.lambda_function_arn
  starting_position       = "LATEST"
  maximum_retry_attempts  = 5
  function_response_types = ["ReportBatchItemFailures"]

  destination_config {
    on_failure {
      destination_arn = module.sqs_s3_audit_dead_letter.queue_arn
    }
  }
}
