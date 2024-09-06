module "label_developers" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  name      = "developers"
  tags      = local.common_tags
  delimiter = "-"
}

resource "aws_iam_group" "developers" {
  name = module.label_developers.id
}
