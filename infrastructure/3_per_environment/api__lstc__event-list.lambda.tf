# ------------------------------------------------------------------------------
# lstc - event-list
# ------------------------------------------------------------------------------

resource "aws_iam_role" "lstc__event-list__role" {
  name = substr("${local.prefix}--api--lstc--event-list", 0, 64)
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Principal = {
          Service = "lambda.amazonaws.com"
        },
        Action = "sts:AssumeRole"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "lstc__event-list__lambda-log" {
  role       = aws_iam_role.lstc__event-list__role.name
  policy_arn = aws_iam_policy.lambda-logs.arn
}

resource "aws_iam_role_policy_attachment" "lstc__event-list__data-read" {
  role       = aws_iam_role.lstc__event-list__role.name
  policy_arn = aws_iam_policy.data--dynamodb-read.arn
}

data "archive_file" "lstc__event-list" {
  type        = "zip"
  source_dir  = "${path.module}/../../api/lstc/event-list/out"
  output_path = "${path.module}/output/lstc__event-list.zip"
}

resource "aws_lambda_function" "lstc__event-list" {
  function_name    = substr("${local.prefix}--api--lstc--event-list", 0, 64)
  runtime          = "provided.al2023"
  handler          = "index.handler"
  role             = aws_iam_role.lstc__event-list__role.arn
  filename         = data.archive_file.lstc__event-list.output_path
  source_code_hash = data.archive_file.lstc__event-list.output_base64sha256
  timeout          = local.lambda_timeout
  memory_size      = 128
  # vpc_config {
  #     subnet_ids         = [ aws_vpc.(none).id ]
  #     security_group_ids = [ aws_subnet.(none)_A.id ]
  # }
  environment {
    variables = {
      PREFIX                       = local.prefix
      Access_Control_Allow_Methods = "GET,OPTIONS"
      Access_Control_Allow_Headers = var.cors-headers
      Access_Control_Allow_Origin  = var.cors-origin
      TABLE_NAME                   = aws_dynamodb_table.data.name
    }
  }
}

resource "aws_cloudwatch_log_group" "lstc__event-list__lambda-log" {
  name              = "/aws/lambda/${local.prefix}_lstc__event-list"
  retention_in_days = local.log_retention_in_days
}

resource "aws_lambda_permission" "lstc__event-list__lambda-permission" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lstc__event-list.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "arn:aws:execute-api:${local.region}:${var.account}:${aws_api_gateway_rest_api.lstc.id}/*/event/{year}"
}