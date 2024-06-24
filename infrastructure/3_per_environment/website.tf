# ------------------------------------------------------------------------------
# ECS Task Definition
# ------------------------------------------------------------------------------

# This policy is used by the container for permissions
resource "aws_iam_role" "api_ecs_web_task" {
  name = "${local.prefix}--web--ecs-task"

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

resource "aws_cloudwatch_log_group" "lstc-web_container" {
  name = "ecs/${local.prefix}--lstc-web--container"
}

data "aws_ecr_repository" "lstc-web" {
  name = "lstc_web"
}

resource "aws_ecs_task_definition" "web" {
  family                   = "${local.prefix}--lstc-web"
  task_role_arn            = aws_iam_role.api_ecs_web_task.arn
  execution_role_arn       = aws_iam_role.api_ecs_execution.arn
  network_mode             = "awsvpc"
  cpu                      = var.web-cpu
  memory                   = var.web-memory
  requires_compatibilities = ["FARGATE"]
  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "ARM64"
  }
  container_definitions = jsonencode(
    [
      {
        "name" : "${local.prefix}--lstc-web--container",
        "image" : "${data.aws_ecr_repository.lstc-web.repository_url}:${var.web-version}",
        "cpu" : "${var.web-cpu}",
        "memory" : "${var.web-memory}",
        "logConfiguration" : {
          "logDriver" : "awslogs",
          "options" : {
            "awslogs-region" : local.region,
            "awslogs-group" : aws_cloudwatch_log_group.lstc-web_container.name,
            "awslogs-create-group" : "true",
            "awslogs-stream-prefix" : "ecs"
          }
        },
        "portMappings" : [
          {
            "appProtocol" : "http",
            "name" : "http",
            "protocol" : "tcp",
            "hostPort" : 80,
            "containerPort" : 80
          }
        ]
      }
  ])
}

# ------------------------------------------------------------------------------
# Security Group (API)
# ------------------------------------------------------------------------------

resource "aws_security_group" "lstc-web" {
  name        = "${local.prefix}--lstc-web"
  description = "Inbound/Outbound traffic for the LSTC WEB"
  vpc_id      = aws_vpc.vpc.id
}

resource "aws_vpc_security_group_ingress_rule" "lstc-web--allow_http" {
  security_group_id = aws_security_group.lstc-web.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "tcp"
  from_port         = 80
  to_port           = 80
}

resource "aws_vpc_security_group_egress_rule" "lstc-web--allow_all" {
  security_group_id = aws_security_group.lstc-web.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "all"
}

# ------------------------------------------------------------------------------
# ECS Service
# ------------------------------------------------------------------------------

resource "aws_ecs_service" "lstc-web" {
  name            = "${local.prefix}--lstc-web"
  cluster         = aws_ecs_cluster.cluster.id
  task_definition = aws_ecs_task_definition.web.arn
  desired_count   = var.web-desired-count
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = [for subnet in aws_subnet.private : subnet.id]
    security_groups  = [aws_security_group.lstc-web.id]
    assign_public_ip = false
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.lstc-web.arn
    container_name   = "${local.prefix}--lstc-web--container"
    container_port   = 80
  }
}
