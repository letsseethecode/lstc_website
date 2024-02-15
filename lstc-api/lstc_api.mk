.PHONY=api api--build api--pack api--publish

SRC_FILES:=./lstc_api/src/main.rs
# GIT_HASH ?= $(shell git log --format="%h" -n 1)

include utils.mk

#
# These are the proper targets for building and publishing the API
#

lstc_api/target/release/lstc_api: $(SRC_FILES)
	cd lstc_api && \
	cargo build --release

.cache/api--pack__%: expect_ECR api--build
	echo "Packing..."
	docker build lstc_api/. -t "lstc_api:${*}" -t "${ECR}/lstc_api:${*}" && \
	touch $@

.cache/api--publish__%: expect_ECR .cache/api--pack__%
	echo "Publishing..."
	docker push "${ECR}/lstc_api:${*}" && \
	touch $@

#
# These are the short-hand targets for the api, which are easier to use
#

api--build: lstc_api/target/release/lstc_api ## Compile the API

api--pack: expect_VERSION .cache/api--pack__$(VERSION) ## Build the Docker container

api--publish: expect_VERSION .cache/api--publish__$(VERSION) ## Publish the API to ECR

api--clean: ## Clean up the API files
	rm -rf lstc_api/target/*
	rm -f .cache/api--*
