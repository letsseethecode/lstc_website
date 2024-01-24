resource "aws_vpc" "vpc" {
  cidr_block = "10.0.0.0/16"
  tags = {
    Name = "${local.prefix} VPC"
  }
}

data "aws_availability_zones" "available" {}

resource "aws_subnet" "public" {
  count = 1
  
  vpc_id            = aws_vpc.vpc.id
  cidr_block        = cidrsubnet(aws_vpc.vpc.cidr_block, 4, count.index)
  availability_zone = element(data.aws_availability_zones.available.names, count.index)

  tags = {
    Name = "${local.prefix} private subnet ${count.index}"
  }
}

resource "aws_subnet" "private" {
  count = 1
  
  vpc_id            = aws_vpc.vpc.id
  cidr_block        = cidrsubnet(aws_vpc.vpc.cidr_block, 4, length(aws_subnet.public) + count.index)
  availability_zone = element(data.aws_availability_zones.available.names, count.index)

  tags = {
    Name = "${local.prefix} public subnet ${count.index}"
  }
}
