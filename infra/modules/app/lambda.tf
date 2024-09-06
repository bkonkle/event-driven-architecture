module "label_http_api" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "http-api"
  tags      = local.common_tags
  delimiter = "-"
}

module "lambda_http_api" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = module.label_http_api.id
  description   = "The HTTP API"
  handler       = "bootstrap"
  runtime       = "provided.al2023"

  source_path = "../../target/lambda/event-driven-architecture"

  attach_policy_statements = true
  policy_statements = {
    dynamodb = {
      effect = "Allow",
      actions = [
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:DescribeStream",
        "dynamodb:ListStreams"
      ]
      resources = [module.dynamodb_event_log.dynamodb_table_stream_arn]
    }
  }

  assume_role_policy_statements = {
    dynamodb = {
      effect = "Allow",
      actions = [
        "sts:AssumeRole"
      ],
      principals = {
        value = {
          type        = "Service"
          identifiers = ["apigateway.amazonaws.com"]
        }
      }
    }
  }
}

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

  attach_dead_letter_policy = true
  dead_letter_target_arn    = module.sqs_publisher_kinesis_dead_letter.queue_arn

  event_source_mapping = {
    dynamodb = {
      event_source_arn           = module.dynamodb_event_log.dynamodb_table_stream_arn
      starting_position          = "LATEST"
      maximum_retry_attempts     = 5
      destination_arn_on_failure = module.sqs_publisher_kinesis_dead_letter.queue_arn
    }
  }

  attach_policy_statements = true
  policy_statements = {
    dynamodb = {
      effect = "Allow",
      actions = [
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:DescribeStream",
        "dynamodb:ListStreams"
      ]
      resources = [module.dynamodb_event_log.dynamodb_table_stream_arn]
    }
    kinesis = {
      effect = "Allow",
      actions = [
        "kinesis:PutRecord",
        "kinesis:PutRecords"
      ],
      resources = [aws_kinesis_stream.event_stream.arn]
    },
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

  attach_dead_letter_policy = true
  dead_letter_target_arn    = module.sqs_projector_s3_audit_dead_letter.queue_arn

  event_source_mapping = {
    kinesis = {
      event_source_arn           = resource.aws_kinesis_stream.event_stream.arn
      starting_position          = "LATEST"
      maximum_retry_attempts     = 5
      function_response_types    = ["ReportBatchItemFailures"]
      destination_arn_on_failure = module.sqs_projector_s3_audit_dead_letter.queue_arn
    }
  }

  attach_policy_statements = true
  policy_statements = {
    kinesis = {
      effect = "Allow",
      actions = [
        "kinesis:GetRecords",
        "kinesis:GetShardIterator",
        "kinesis:DescribeStream",
        "kinesis:DescribeStreamSummary",
        "kinesis:ListShards",
        "kinesis:ListStreams"
      ],
      resources = [aws_kinesis_stream.event_stream.arn]
    },
    s3 = {
      effect = "Allow",
      actions = [
        "s3:PutObject",
        "s3:ListBucket",
        "s3:GetObject"
      ],
      resources = [
        "arn:aws:s3:::${module.s3_event_audit.s3_bucket_id}/*",
        "arn:aws:s3:::${module.s3_event_audit.s3_bucket_id}"
      ]
    }
  }
}
