variable "api-version" {
  type        = string
  description = "The container version to deploy from the ECR"
}

variable "web-version" {
  type        = string
  description = "The container version to deploy from the ECR"
}

variable "api-desired-count" {
  type        = number
  description = "The number of APIs to be deployed"
}

variable "api-cpu" {
  type        = number
  description = "Size of the CPU for the API (256 = 0.25 vCPU)"
}

variable "api-memory" {
  type        = number
  description = "Memory for API in MB"
}

variable "web-desired-count" {
  type        = number
  description = "The number of WEB servers to be deployed"
}

variable "web-cpu" {
  type        = number
  description = "Size of the CPU for the WEB (256 = 0.25 vCPU)"
}

variable "web-memory" {
  type        = number
  description = "Memory for WEB in MB"
}
