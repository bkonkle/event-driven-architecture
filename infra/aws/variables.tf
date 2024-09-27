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

variable "developers" {
  type = map(object({
    path                 = optional(string, "/")
    permissions_boundary = optional(string, "")
    login_profile        = optional(bool, false)
    pgp_key              = optional(string, "")
    access_key           = optional(bool, false)
  }))
}

locals {
  account_id = data.aws_caller_identity.current.account_id

  common_tags = {
    ProvisionedBy = "terraform"
  }
}
