module "sqs_publisher_kinesis_dead_letter" {
  source = "terraform-aws-modules/sqs/aws"

  name = "publisher-kinesis-dead-letter"
}

module "sqs_s3_audit_dead_letter" {
  source = "terraform-aws-modules/sqs/aws"

  name = "s3-audit-dead-letter"
}
