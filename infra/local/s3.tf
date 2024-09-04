module "s3_event_audit" {
  source = "terraform-aws-modules/s3-bucket/aws"

  bucket = "event-audit"
  acl    = "private"

  control_object_ownership = true
  object_ownership         = "ObjectWriter"
}
