provider "aws" {
  region = local.region
  default_tags {
    tags = {
      project = local.project
      account = local.environment
    }
  }
}

terraform {
  backend "s3" {
    encrypt              = false
    region               = "eu-west-2"
    bucket               = "lstc-terraform-state2"
    dynamodb_table       = "lstc-terraform-lock"
    key                  = "lstc-website-environment"
    workspace_key_prefix = "lstc-website"
  }
}
