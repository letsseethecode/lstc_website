# We should create a custom VPC, rather than using the default
resource "aws_vpc" "vpc" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true
  tags = {
    Name = "${local.prefix} VPC"
  }
}

# Lookup the availability zones rather than specifying them:
#   eu-west-2a, eu-west-2b, eu-west-2c
data "aws_availability_zones" "available" {}

# ------------------------------------------------------------------------------
# A public subnet(s)
# ------------------------------------------------------------------------------

resource "aws_subnet" "public" {
  count = 2

  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = cidrsubnet(aws_vpc.vpc.cidr_block, 4, count.index)
  availability_zone       = element(data.aws_availability_zones.available.names, count.index)
  map_public_ip_on_launch = true

  tags = {
    Name = "${local.prefix} public subnet ${count.index}"
  }
}

# ------------------------------------------------------------------------------
# private subnet(s)
# ------------------------------------------------------------------------------

resource "aws_subnet" "private" {
  count = 2

  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = cidrsubnet(aws_vpc.vpc.cidr_block, 4, length(aws_subnet.public) + count.index)
  availability_zone       = element(data.aws_availability_zones.available.names, count.index)
  map_public_ip_on_launch = false

  tags = {
    Name = "${local.prefix} private subnet ${count.index}"
  }
}

# ------------------------------------------------------------------------------
# Gateways
# ------------------------------------------------------------------------------

resource "aws_internet_gateway" "internet_gateway" {
  vpc_id = aws_vpc.vpc.id
  tags = {
    Name = "${local.prefix}--ig"
  }
}

resource "aws_eip" "nat_eip" {
  domain     = "vpc"
  depends_on = [aws_internet_gateway.internet_gateway]
}

resource "aws_nat_gateway" "nat" {
  allocation_id = aws_eip.nat_eip.id
  subnet_id     = element(aws_subnet.public.*.id, 0)
  depends_on    = [aws_internet_gateway.internet_gateway]
  tags = {
    Name        = "${local.prefix}--nat"
    Environment = "${local.environment}"
  }
}

# ------------------------------------------------------------------------------
# public route table
# ------------------------------------------------------------------------------

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.vpc.id
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.internet_gateway.id
  }
  tags = {
    Name = "${local.prefix}--public"
  }
}

# ------------------------------------------------------------------------------
# private route table
# ------------------------------------------------------------------------------

resource "aws_route_table" "private" {
  vpc_id = aws_vpc.vpc.id
  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = aws_nat_gateway.nat.id
  }
  tags = {
    Name = "${local.prefix}--private"
  }
}

# ------------------------------------------------------------------------------
# Associate routing tables to subnets
# ------------------------------------------------------------------------------

resource "aws_route_table_association" "public" {
  count          = length(aws_subnet.public)
  subnet_id      = aws_subnet.public[count.index].id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "private" {
  count          = length(aws_subnet.private)
  subnet_id      = aws_subnet.private[count.index].id
  route_table_id = aws_route_table.private.id
}
