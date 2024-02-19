resource "aws_route53_zone" "primary" {
  name = "letsseethecode.com"
}

# resource "aws_route53_record" "apex" {
#   zone_id = aws_route53_zone.primary.id
#   name    = aws_route53_zone.primary.name
#   type    = "CNAME"
#   ttl     = 5
#   records = [
#     "letsseethecode.github.io"
#   ]
# }

resource "aws_route53_record" "www" {
  zone_id = aws_route53_zone.primary.id
  name    = "www."
  type    = "CNAME"
  ttl     = 5
  records = [
    "letsseethecode.github.io"
  ]
}
