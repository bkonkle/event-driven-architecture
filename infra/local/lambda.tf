module "lambda_publisher_kinesis" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "publisher-kinesis"
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
}
