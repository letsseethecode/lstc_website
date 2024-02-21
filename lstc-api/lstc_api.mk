.PHONY=api api--install api--build api--pack api--publish

SRC_FILES:=$(shell find ./lstc-api -name "*.rs")
# GIT_HASH ?= $(shell git log --format="%h" -n 1)

include utils.mk

#
# These are the proper targets for building and publishing the API
#

lstc-api/target/release/lstc-api: $(SRC_FILES)
	cd lstc-api && \
	cargo build --release

.cache/api--pack__%: expect_ECR api--build
	echo "Packing..."
	docker build lstc-api/. -t "lstc-api:${*}" -t "${ECR}/lstc-api:${*}" && \
	touch $@

.cache/api--publish__%: expect_ECR .cache/api--pack__%
	echo "Publishing..."
	docker push "${ECR}/lstc-api:${*}" && \
	touch $@

#
# These are the short-hand targets for the api, which are easier to use
#

api--install:															## Install pre-requisites for lstc-api
	echo "Not implemented"

api--clean:																## Clean up the API files
	rm -rf lstc-api/target/* &
	rm -f .cache/api--*

api--build: lstc-api/target/release/lstc-api							## Compile the API

api--pack: expect_VERSION api-build .cache/api--pack__$(VERSION)		## Build the Docker container

api--publish: expect_VERSION api-pack .cache/api--publish__$(VERSION)	## Publish the API to ECR
