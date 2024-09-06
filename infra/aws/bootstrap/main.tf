provider "aws" {
  region = var.region
}

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 5.65"
    }
  }
}

module "dynamodb_terraform_locks" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name     = "${var.namespace}-tf-locks"
  hash_key = "LockID"

  attributes = [
    {
      name = "LockID"
      type = "S"
    }
  ]
}

module "s3_terraform_state" {
  source = "terraform-aws-modules/s3-bucket/aws"

  bucket = "${var.namespace}-${var.region}-tf-state"
  acl    = "private"

  control_object_ownership = true
  object_ownership         = "ObjectWriter"

  server_side_encryption_configuration = {
    rule = {
      apply_server_side_encryption_by_default = {
        sse_algorithm = "AES256"
      }
    }
  }
}
