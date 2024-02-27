data "aws_route53_zone" "primary" {
  name = "letsseethecode.com"
}

resource "aws_route53_record" "lb" {
  zone_id         = data.aws_route53_zone.primary.id
  name            = "${local.environment}.${data.aws_route53_zone.primary.name}"
  type            = "A"
  allow_overwrite = true

  alias {
    name                   = aws_lb.lstc.dns_name
    zone_id                = aws_lb.lstc.zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "lb-dvo" {
  for_each = {
    for dvo in aws_acm_certificate.lb.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }

  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = data.aws_route53_zone.primary.id
}

resource "aws_acm_certificate" "lb" {
  domain_name               = "${local.environment}.${data.aws_route53_zone.primary.name}"
  subject_alternative_names = ["${local.environment}.${data.aws_route53_zone.primary.name}"]
  validation_method         = "DNS"
  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_acm_certificate_validation" "validation" {
  certificate_arn         = aws_acm_certificate.lb.arn
  validation_record_fqdns = [for record in aws_route53_record.lb-dvo : record.fqdn]
}
