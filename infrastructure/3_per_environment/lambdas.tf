locals {
  lambda_timeout        = 30
  log_retention_in_days = 1
}

# ------------------------------------------------------------------------------
# Policy: lambda-logs
# ------------------------------------------------------------------------------

data "aws_iam_policy_document" "lambda-logs" {
  statement {
    sid    = "LogGroups"
    effect = "Allow"
    actions = [
      "logs:CreateLogGroup"
    ]
    resources = [
      "arn:aws:logs:eu-west-2:${var.account}:*"
    ]
  }
  statement {
    sid    = "LogStreams"
    effect = "Allow"
    actions = [
      "logs:CreateLogStream",
      "logs:PutLogEvents"
    ]
    resources = [
      "arn:aws:logs:eu-west-2:${var.account}:log-group:/aws/lambda/${local.prefix}_*:*"
    ]
  }
}
resource "aws_iam_policy" "lambda-logs" {
  name   = "${local.prefix}_lambda-logs"
  policy = data.aws_iam_policy_document.lambda-logs.json
}
