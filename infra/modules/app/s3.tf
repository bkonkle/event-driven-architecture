module "label_event_audit" {
  source      = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace   = var.namespace
  environment = var.region
  stage       = var.environment
  name        = "event-audit"
  tags        = local.common_tags
  delimiter   = "-"
}

module "s3_event_audit" {
  source = "terraform-aws-modules/s3-bucket/aws"

  bucket = module.label_event_audit.id
  acl    = "private"

  control_object_ownership = true
  object_ownership         = "ObjectWriter"
}
