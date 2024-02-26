data "aws_route53_zone" "primary" {
  name = "letsseethecode.com"
}

resource "aws_route53_record" "web" {
  zone_id = data.aws_route53_zone.primary.id
  name    = "${local.environment}.${data.aws_route53_zone.primary.name}"
  type    = "A"
  alias {
    name                   = aws_s3_bucket_website_configuration.website.website_domain
    zone_id                = aws_s3_bucket.website.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "api" {
  zone_id = data.aws_route53_zone.primary.id
  name    = "api-${local.environment}.${data.aws_route53_zone.primary.name}"
  type    = "A"

  alias {
    name                   = aws_lb.lstc.dns_name
    zone_id                = aws_lb.lstc.zone_id
    evaluate_target_health = false
  }
}
