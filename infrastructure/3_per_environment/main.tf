provider "aws" {
  region = local.region
  default_tags {
    tags = {
      project     = local.project
      environment = local.environment
    }
  }
}

terraform {
  backend "s3" {
    encrypt              = false
    region               = "us-east-1"
    bucket               = "lstc-terraform-state"
    dynamodb_table       = "lstc-terraform-lock"
    key                  = "lstc-website-environment"
    workspace_key_prefix = "lstc-website"
  }
}
