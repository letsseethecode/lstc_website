.PHONY: help install clean build pack publish init plan apply destroy
.DEFAULT_GOAL := help

SCOPE ?= per_environment

MAKE_FILES := $(shell find . -name "*.mk")
include $(MAKE_FILES)

BUILD_OUTPUT := $(WEB_OUTPUT)

install: web--install									## Install the pre-requisites

clean: per_environment--clean api--clean web--clean		## Remove all the compiled artifacts
	rm .cache/*

build: $(BUILD_OUTPUT) 									## Build the project

pack: web--pack											## Pack assets, ready for publishing

publish: web--publish 									## Publish assets to repositories (e.g. ECR)

init: workspace $(SCOPE)--init 							## Initialise the Terraform

plan: $(SCOPE)--plan 									## Prepare the project for deployment

apply: $(SCOPE)--apply 									## Deploy the project

destroy: $(SCOPE)--destroy 								## Destroy the project

help: 													## This help message
	@echo "Available targets:"
	@grep -E --no-filename '^[a-zA-Z-]+:.*?## .*$$' Makefile $(MAKE_FILES) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
