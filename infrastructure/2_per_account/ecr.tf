resource "aws_ecr_repository" "lstc" {
  name                 = "lstc_api"
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}

output "ecr_url" {
  value = aws_ecr_repository.lstc.repository_url
}
