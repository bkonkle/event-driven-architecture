data "aws_caller_identity" "current" {}

data "aws_partition" "current" {}

variable "namespace" {
  type    = string
  default = "event-driven"
}

variable "username" {
  description = "Desired name for the IAM user"
  type        = string
}

variable "path" {
  description = "Desired path for the IAM user"
  type        = string
  default     = "/"
}

variable "permissions_boundary" {
  description = "The ARN of the policy that is used to set the permissions boundary for the user."
  type        = string
  default     = ""
}

variable "login_profile" {
  type    = bool
  default = false
}

variable "pgp_key" {
  description = "Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:username`. Used to encrypt password and access key. `pgp_key` is required when `create_iam_user_login_profile` is set to `true`"
  type        = string
  default     = ""
}

variable "access_key" {
  type    = bool
  default = false
}

variable "groups" {
  type    = list(string)
  default = []
}

locals {
  account_id = data.aws_caller_identity.current.account_id

  common_tags = {
    ProvisionedBy = "terraform"
  }
}
