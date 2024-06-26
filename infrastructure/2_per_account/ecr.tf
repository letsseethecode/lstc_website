resource "aws_ecr_repository" "lstc-web" {
  name                 = "lstc_web"
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}

output "ecr_web_url" {
  value = aws_ecr_repository.lstc-web.repository_url
}
