locals {
  region      = "eu-west-2"
  project     = "lstc_website"
  environment = terraform.workspace
  prefix      = "${local.project}--${local.environment}"

  content_types = {
    css  = "text/css"
    html = "text/html"
    js   = "application/javascript"
    json = "application/json"
    txt  = "text/plain"
    wasm = "application/wasm"
  }
}
