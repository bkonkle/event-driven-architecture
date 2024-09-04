module "dynamodb_event_log" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name      = "event_log"
  hash_key  = "AggregateTypeAndId"
  range_key = "AggregateIdSequence"

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

module "dynamodb_event_snapshots" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name     = "event_snapshots"
  hash_key = "AggregateTypeAndId"

  attributes = [
    {
      name = "AggregateTypeAndId"
      type = "S"
    }
  ]
}

module "dynamodb_tasks_view" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name     = "tasks_view"
  hash_key = "ViewId"

  attributes = [
    {
      name = "ViewId"
      type = "S"
    }
  ]
}
