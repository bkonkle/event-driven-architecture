module "label_kinesis_dead_letter" {
  source     = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace  = var.namespace
  stage      = var.environment
  name       = "publisher-kinesis"
  attributes = ["dead-letter"]
  tags       = local.common_tags
  delimiter  = "-"
}

module "sqs_publisher_kinesis_dead_letter" {
  source = "terraform-aws-modules/sqs/aws"

  name = module.label_kinesis_dead_letter.id
}

module "label_s3_audit_dead_letter" {
  source     = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace  = var.namespace
  stage      = var.environment
  name       = "s3-audit"
  attributes = ["dead-letter"]
  tags       = local.common_tags
  delimiter  = "-"
}

module "sqs_s3_audit_dead_letter" {
  source = "terraform-aws-modules/sqs/aws"

  name = module.label_s3_audit_dead_letter.id
}
