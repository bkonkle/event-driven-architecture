data "aws_caller_identity" "current" {}

variable "region" {
  type    = string
  default = "us-west-2"
}

variable "namespace" {
  type    = string
  default = "event-driven"
}

variable "environment" {
  type    = string
  default = "dev"
}

locals {
  account_id = data.aws_caller_identity.current.account_id

  common_tags = {
    ProvisionedBy = "terraform"
  }
}
