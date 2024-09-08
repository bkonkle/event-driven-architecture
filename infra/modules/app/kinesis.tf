module "label_event_stream" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "event-stream"
  tags      = local.common_tags
  delimiter = "-"
}

resource "aws_kinesis_stream" "event_stream" {
  name                      = module.label_event_stream.id
  shard_count               = var.environment == "prod" ? null : 1
  retention_period          = var.environment == "prod" ? 48 : 24
  enforce_consumer_deletion = true
  encryption_type           = "KMS"
  kms_key_id                = "alias/aws/kinesis"

  shard_level_metrics = [
    "IncomingBytes",
    "OutgoingBytes",
  ]

  stream_mode_details {
    stream_mode = var.environment == "prod" ? "ON_DEMAND" : "PROVISIONED"
  }
}
