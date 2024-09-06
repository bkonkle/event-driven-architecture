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
    "ANY /" = {
      detailed_metrics_enabled = false

      integration = {
        type                   = "AWS_PROXY"
        uri                    = module.lambda_http_api.lambda_function_arn
        payload_format_version = "2.0"
        timeout_milliseconds   = 12000
      }
    }

    "$default" = {
      integration = {
        uri = module.lambda_http_api.lambda_function_arn

        response_parameters = [
          {
            status_code = 500
            mappings = {
              "append:header.header1" = "$context.requestId"
              "overwrite:statuscode"  = "403"
            }
          },
          {
            status_code = 404
            mappings = {
              "append:header.error" = "$stageVariables.environmentId"
            }
          }
        ]
      }
    }
  }

  stage_name = var.environment

  stage_access_log_settings = {
    create_log_group            = true
    log_group_retention_in_days = 7
    format = jsonencode({
      context = {
        domainName              = "$context.domainName"
        integrationErrorMessage = "$context.integrationErrorMessage"
        protocol                = "$context.protocol"
        requestId               = "$context.requestId"
        requestTime             = "$context.requestTime"
        responseLength          = "$context.responseLength"
        routeKey                = "$context.routeKey"
        stage                   = "$context.stage"
        status                  = "$context.status"
        error = {
          message      = "$context.error.message"
          responseType = "$context.error.responseType"
        }
        identity = {
          sourceIP = "$context.identity.sourceIp"
        }
        integration = {
          error             = "$context.integration.error"
          integrationStatus = "$context.integration.integrationStatus"
        }
      }
    })
  }

  stage_default_route_settings = {
    detailed_metrics_enabled = true
    throttling_burst_limit   = 100
    throttling_rate_limit    = 100
  }

  tags = local.common_tags
}
