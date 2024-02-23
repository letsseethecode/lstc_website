resource "aws_iam_role" "api_ecs_execution" {
  name = "${local.prefix}--api--ecs-execution"

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

resource "aws_iam_role_policy_attachment" "ecs-task-execution-role-policy-attachment" {
  role       = aws_iam_role.api_ecs_execution.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

# resource "aws_iam_role_policy_attachment" "task_s3" {
#   role       = "${aws_iam_role.ecs_task_role.name}"
#   policy_arn = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
# }

resource "aws_ecs_cluster" "cluster" {
  name = "${local.prefix}--cluster"
}

data "aws_ecr_repository" "lstc-api" {
  name = "lstc_api"
}

resource "aws_cloudwatch_log_group" "lstc-api_container" {
  name = "ecs/${local.prefix}--lstc-api--container"
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

resource "aws_ecs_service" "lstc-api" {
  name            = "${local.prefix}--lstc-api"
  cluster         = aws_ecs_cluster.cluster.id
  task_definition = aws_ecs_task_definition.definition.arn
  desired_count   = var.api-desired-count
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = [for subnet in aws_subnet.public : subnet.id]
    security_groups  = [aws_security_group.lstc-api.id]
    assign_public_ip = true # TODO fix this
  }
}
