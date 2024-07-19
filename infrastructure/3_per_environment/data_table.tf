resource "aws_dynamodb_table" "data" {
  name         = "${local.prefix}--data"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "pk"
  range_key    = "sk"

  attribute {
    name = "pk"
    type = "S"
  }

  attribute {
    name = "sk"
    type = "S"
  }

  tags = {
    Name        = "${local.prefix}--data"
    Environment = "${local.environment}"
  }
}

resource "aws_iam_policy" "data--dynamodb-read" {
  name        = "${local.prefix}--data--dynamodb-read"
  path        = "/"
  description = "Query the data table"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "dynamodb:Query",
          "dynamodb:Scan",
          "dynamodb:GetItem",
        ],
        Resource = [
          aws_dynamodb_table.data.arn
        ]
      }
    ]
  })
}

resource "aws_iam_policy" "data--dynamodb-write" {
  name        = "${local.prefix}--data--dynamodb-write"
  path        = "/"
  description = "Query the data table"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
        ],
        Resource = [
          aws_dynamodb_table.data.arn
        ]
      }
    ]
  })
}
