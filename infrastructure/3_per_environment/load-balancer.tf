# ------------------------------------------------------------------------------
# Target Groups
# ------------------------------------------------------------------------------

resource "aws_lb_target_group" "lstc-web" {
  name        = "${replace(local.prefix, "_", "-")}--lstc-web"
  port        = 80
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = aws_vpc.vpc.id
}

resource "aws_lb_target_group" "lstc-api" {
  name        = "${replace(local.prefix, "_", "-")}--lstc-api"
  port        = 8080
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = aws_vpc.vpc.id

  health_check {
    path = "/health"
  }
}

# ------------------------------------------------------------------------------
# Load Balancer
# ------------------------------------------------------------------------------

resource "aws_lb" "lstc" {
  name               = "${replace(local.prefix, "_", "-")}--lstc-lb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.lstc-api-lb.id]
  subnets            = [for subnet in aws_subnet.public : subnet.id]

  tags = {
    Name = "${local.prefix}--lstc-api-lb"
  }
}

# ------------------------------------------------------------------------------
# PORT 80 listener
# ------------------------------------------------------------------------------

resource "aws_lb_listener" "lstc-80" {
  load_balancer_arn = aws_lb.lstc.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.lstc-web.arn
  }
}

# ------------------------------------------------------------------------------
# API redirect
# ------------------------------------------------------------------------------

resource "aws_lb_listener_rule" "lstc-80--api" {
  listener_arn = aws_lb_listener.lstc-80.arn
  priority     = 1
  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.lstc-api.arn
  }
  condition {
    path_pattern {
      values = ["/api/*"]
    }
  }
  tags = {
    Name = "API"
  }
}

