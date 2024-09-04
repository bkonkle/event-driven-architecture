resource "aws_kinesis_stream" "event_stream" {
  name                      = "event-stream"
  retention_period          = 24
  enforce_consumer_deletion = true
  encryption_type           = "KMS"
  kms_key_id                = "alias/aws/kinesis"

  shard_level_metrics = [
    "IncomingBytes",
    "OutgoingBytes",
  ]

  stream_mode_details {
    stream_mode = "ON_DEMAND"
  }
}
