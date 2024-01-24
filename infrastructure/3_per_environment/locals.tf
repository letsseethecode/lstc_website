locals {
  region      = "eu-west-2"
  project     = "lstc_website"
  environment = terraform.workspace
  prefix      = "${local.project}-${local.environment}"
}
