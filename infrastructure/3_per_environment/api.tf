# ------------------------------------------------------------------------------
# ECS Task Definition
# ------------------------------------------------------------------------------

# This policy is used by the container for permissions
resource "aws_iam_role" "api_ecs_task" {
  name = "${local.prefix}--api--ecs-task"

  assume_role_policy = jsonencode(
    {
      "Version" : "2012-10-17",
      "Statement" : [
        {
          "Action" : "sts:AssumeRole",
          "Principal" : {
            "Service" : "ecs-tasks.amazonaws.com"
          },
          "Effect" : "Allow",
          "Sid" : ""
        }
      ]
    }
  )
}

resource "aws_cloudwatch_log_group" "lstc-api_container" {
  name = "ecs/${local.prefix}--lstc-api--container"
}

data "aws_ecr_repository" "lstc-api" {
  name = "lstc_api"
}

resource "aws_ecs_task_definition" "definition" {
  family                   = "${local.prefix}--lstc-api"
  task_role_arn            = aws_iam_role.api_ecs_task.arn
  execution_role_arn       = aws_iam_role.api_ecs_execution.arn
  network_mode             = "awsvpc"
  cpu                      = var.api-cpu
  memory                   = var.api-memory
  requires_compatibilities = ["FARGATE"]
  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "ARM64"
  }
  container_definitions = jsonencode(
    [
      {
        "name" : "${local.prefix}--lstc-api--container",
        "image" : "${data.aws_ecr_repository.lstc-api.repository_url}:${var.api-version}",
        "cpu" : "${var.api-cpu}",
        "memory" : "${var.api-memory}",
        "logConfiguration" : {
          "logDriver" : "awslogs",
          "options" : {
            "awslogs-region" : local.region,
            "awslogs-group" : aws_cloudwatch_log_group.lstc-api_container.name,
            "awslogs-create-group" : "true",
            "awslogs-stream-prefix" : "ecs"
          }
        },
        "portMappings" : [
          {
            "appProtocol" : "http",
            "name" : "http",
            "protocol" : "tcp",
            "hostPort" : 8080,
            "containerPort" : 8080
          }
        ]
      }
  ])
}

# ------------------------------------------------------------------------------
# Security Group (API LB)
# ------------------------------------------------------------------------------

resource "aws_security_group" "lstc-api-lb" {
  name        = "${local.prefix}--lstc-api-lb"
  description = "Inbound/Outbound traffic for the LSTC API LB"
  vpc_id      = aws_vpc.vpc.id
}

resource "aws_vpc_security_group_ingress_rule" "lstc-api-lb--allow_http" {
  security_group_id = aws_security_group.lstc-api-lb.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "tcp"
  from_port         = 80
  to_port           = 80
}

resource "aws_vpc_security_group_ingress_rule" "lstc-api-lb--allow_https" {
  security_group_id = aws_security_group.lstc-api-lb.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "tcp"
  from_port         = 443
  to_port           = 443
}

resource "aws_vpc_security_group_egress_rule" "lstc-api-lb--allow_all" {
  security_group_id = aws_security_group.lstc-api-lb.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "all"
}

# ------------------------------------------------------------------------------
# Security Group (API)
# ------------------------------------------------------------------------------

resource "aws_security_group" "lstc-api" {
  name        = "${local.prefix}--lstc-api"
  description = "Inbound/Outbound traffic for the LSTC API"
  vpc_id      = aws_vpc.vpc.id
}

resource "aws_vpc_security_group_ingress_rule" "lstc-api--allow_http" {
  security_group_id = aws_security_group.lstc-api.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "tcp"
  from_port         = 8080
  to_port           = 8080
}

resource "aws_vpc_security_group_egress_rule" "lstc-api--allow_all" {
  security_group_id = aws_security_group.lstc-api.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "all"
}

# ------------------------------------------------------------------------------
# ECS Service
# ------------------------------------------------------------------------------

resource "aws_ecs_service" "lstc-api" {
  name            = "${local.prefix}--lstc-api"
  cluster         = aws_ecs_cluster.cluster.id
  task_definition = aws_ecs_task_definition.definition.arn
  desired_count   = var.api-desired-count
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = [for subnet in aws_subnet.private : subnet.id]
    security_groups  = [aws_security_group.lstc-api.id]
    assign_public_ip = false
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.lstc-api.arn
    container_name   = "${local.prefix}--lstc-api--container"
    container_port   = 8080
  }
}
