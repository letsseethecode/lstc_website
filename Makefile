.PHONY := help
.DEFAULT_GOAL := help

ACCOUNT ?= default
ENV ?= dev

MAKE_FILES := $(shell find . -name "*.mk")
include $(MAKE_FILES)

install: api--install web--install

clean: per_environment--clean api--clean web--clean 			## Remove all the compiled artifacts

build: api--build web--build 			## Build the project

pack: api--pack 						## Pack assets, ready for publishing

publish: api--publish 					## Publish assets to repositories (e.g. ECR)

workspace:
	terraform workspace select ${ENV} || terraform workspace new ${ENV}

init: workspace per_environment--init 	## Initialise the Terraform

plan: per_environment--plan 			## Prepare the project for deployment

apply: per_environment--apply 			## Deploy the project

destroy: per_environment--destroy 		## Destroy the project

help: 									## This help message
	@echo "Available targets:"
	@grep -E --no-filename '^[a-zA-Z-]+:.*?## .*$$' Makefile $(MAKE_FILES) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-42s\033[0m %s\n", $$1, $$2}'
