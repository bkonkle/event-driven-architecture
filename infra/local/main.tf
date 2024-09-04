/**
* Localstack terraform aws provider.
* see: https://registry.terraform.io/providers/hashicorp/aws/latest/docs/guides/custom-service-endpoints#localstack
*/
provider "aws" {
  access_key                  = "test"
  secret_key                  = "test"
  region                      = "us-west-2"
  s3_use_path_style           = false
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {
    cloudwatch     = var.base_endpoint
    cloudwatchlogs = var.base_endpoint
    dynamodb       = var.base_endpoint
    eventbridge    = var.base_endpoint
    firehose       = var.base_endpoint
    iam            = var.base_endpoint
    kinesis        = var.base_endpoint
    kms            = var.base_endpoint
    lambda         = var.base_endpoint
    s3             = var.base_s3_endpoint
    secretsmanager = var.base_endpoint
    sns            = var.base_endpoint
    sqs            = var.base_endpoint
    stepfunctions  = var.base_endpoint
    sts            = var.base_endpoint
  }
}

/**
* local backend
*/
terraform {
  backend "local" {}

  required_version = "~> 1.9.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 5.65"
    }
  }
}

locals {
  region = "us-west-2"
}
