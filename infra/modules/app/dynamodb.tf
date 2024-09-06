module "label_event_log" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "event-log"
  tags      = local.common_tags
  delimiter = "-"
}

module "dynamodb_event_log" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name             = module.label_event_log.id
  hash_key         = "AggregateTypeAndId"
  range_key        = "AggregateIdSequence"
  stream_enabled   = true
  stream_view_type = "NEW_IMAGE"

  attributes = [
    {
      name = "AggregateTypeAndId"
      type = "S"
    },
    {
      name = "AggregateIdSequence"
      type = "N"
    }
  ]
}

module "label_event_snapshots" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "event-snapshots"
  tags      = local.common_tags
  delimiter = "-"
}

module "dynamodb_event_snapshots" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name     = module.label_event_snapshots.id
  hash_key = "AggregateTypeAndId"

  attributes = [
    {
      name = "AggregateTypeAndId"
      type = "S"
    }
  ]
}

module "label_tasks_view" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "tasks-view"
  tags      = local.common_tags
  delimiter = "-"
}

module "dynamodb_tasks_view" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name     = module.label_tasks_view.id
  hash_key = "ViewId"

  attributes = [
    {
      name = "ViewId"
      type = "S"
    }
  ]
}
