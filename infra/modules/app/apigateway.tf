module "label_api_gateway" {
  source    = "git::https://github.com/cloudposse/terraform-null-label.git?ref=tags/0.25.0"
  namespace = var.namespace
  stage     = var.environment
  name      = "http"
  tags      = local.common_tags
  delimiter = "-"
}

module "api_gateway" {
  source = "terraform-aws-modules/apigateway-v2/aws"

  name               = module.label_api_gateway.id
  description        = "API Gateway for the HTTP API"
  protocol_type      = "HTTP"
  create_domain_name = false

  routes = {
    "$default" = {
      integration = {
        type = "AWS_PROXY"
        uri  = module.lambda_http_api.lambda_function_arn
      }
    }
  }

  tags = local.common_tags
}
