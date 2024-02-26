data "aws_route53_zone" "primary" {
  name = "letsseethecode.com"
}

resource "aws_route53_record" "lb" {
  zone_id = data.aws_route53_zone.primary.id
  name    = "${local.environment}.${data.aws_route53_zone.primary.name}"
  type    = "A"

  alias {
    name                   = aws_lb.lstc.dns_name
    zone_id                = aws_lb.lstc.zone_id
    evaluate_target_health = false
  }
}
