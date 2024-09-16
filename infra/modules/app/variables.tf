variable "account_id" {
  type    = string
  default = null
}

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

variable "enable_api_gateway" {
  type    = bool
  default = true
}

locals {
  common_tags = {
    ProvisionedBy = "terraform"
  }
}
