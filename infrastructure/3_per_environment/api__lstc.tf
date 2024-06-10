resource "aws_api_gateway_rest_api" "lstc" {
  name        = "${local.prefix}--lstc"
  description = "Terraform Serverless Application Example"
  body = templatefile(
    "../../api/lstc/swagger.yaml",
    {
      environment                  = local.environment
      method_event-list            = "arn:aws:apigateway:eu-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:eu-west-2:${var.account}:function:${local.prefix}--api--lstc--event-list/invocations"
      method_event-get             = "arn:aws:apigateway:eu-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:eu-west-2:${var.account}:function:${local.prefix}--api--lstc--event-get/invocations"
      Access_Control_Allow_Headers = var.cors-headers
      Access_Control_Allow_Origin  = var.cors-origin
    }
  )
}

resource "aws_api_gateway_deployment" "lstc_production" {
  rest_api_id = aws_api_gateway_rest_api.lstc.id
  triggers = {
    redeployment = sha1(jsonencode(aws_api_gateway_rest_api.lstc.body))
  }
  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "production" {
  deployment_id = aws_api_gateway_deployment.lstc_production.id
  rest_api_id   = aws_api_gateway_rest_api.lstc.id
  stage_name    = "production"
}

output "api_url" {
  value = aws_api_gateway_deployment.lstc_production.invoke_url
}
