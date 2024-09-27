module "users" {
  source = "../modules/user"

  for_each = var.developers

  username             = each.key
  path                 = each.value.path
  permissions_boundary = each.value.permissions_boundary
  login_profile        = each.value.login_profile
  pgp_key              = each.value.pgp_key
  access_key           = each.value.access_key
  groups               = [aws_iam_group.developers.name]
}
