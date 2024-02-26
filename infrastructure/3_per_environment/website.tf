resource "aws_s3_bucket" "website" {
  bucket = "${local.environment}.letsseethecode.com"
}

resource "aws_s3_bucket_public_access_block" "website" {
  bucket = aws_s3_bucket.website.id

  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_policy" "website" {
  bucket = aws_s3_bucket.website.id
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "PublicReadGetObject",
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : [
          "s3:GetObject"
        ],
        "Resource" : [
          "arn:aws:s3:::${aws_s3_bucket.website.bucket}/*"
        ]
      }
    ]
  })
}

resource "aws_s3_bucket_website_configuration" "website" {
  bucket = aws_s3_bucket.website.id

  index_document {
    suffix = "index.html"
  }

  #   error_document {
  #     key = "error.html"
  #   }
}

resource "aws_s3_object" "file" {
  for_each     = fileset("../../lstc-web/dist/", "**")
  bucket       = aws_s3_bucket.website.bucket
  key          = each.value
  source       = "../../lstc-web/dist/${each.value}"
  etag         = filemd5("../../lstc-web/dist/${each.value}")
  content_type = lookup(local.content_types, element(split(".", each.value), length(split(".", each.value)) - 1), "text/plain")
}
