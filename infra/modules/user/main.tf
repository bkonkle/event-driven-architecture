module "label" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  name      = var.username
  tags      = local.common_tags
  delimiter = "-"
}

resource "aws_iam_user" "user" {
  name                 = module.label.id
  path                 = var.path
  permissions_boundary = var.permissions_boundary
  tags                 = module.label.tags
}

resource "aws_iam_user_login_profile" "user" {
  count = var.login_profile ? 1 : 0

  user                    = aws_iam_user.user.name
  pgp_key                 = var.pgp_key
  password_length         = 20
  password_reset_required = true
}

resource "aws_iam_access_key" "user" {
  count = var.access_key && var.pgp_key != "" ? 1 : 0

  user    = aws_iam_user.user.name
  pgp_key = var.pgp_key
}

resource "aws_iam_access_key" "user_no_pgp" {
  count = var.access_key && var.pgp_key == "" ? 1 : 0

  user = aws_iam_user.user.name
}

resource "aws_iam_user_group_membership" "user" {
  count = length(var.groups) > 0 ? 1 : 0

  user   = aws_iam_user.user.name
  groups = var.groups
}
