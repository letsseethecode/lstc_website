.PHONY=api api--install api--build api--pack api--publish

SRC_FILES := $(shell find ./lstc-api -name "*.rs")
API_VERSION ?= $(shell grep -E "^version" lstc-api/Cargo.toml | grep -Eo "[0-9+]\.[0-9+]\.[0-9+]")
ECR_URL := $(shell cat infrastructure/2_per_account/output.json | jq -r .ecr_url.value)

include utils.mk

#
# These are the proper targets for building and publishing the API
#

lstc-api/target/release/lstc-api: $(SRC_FILES)
	cd lstc-api && \
	cargo build --release

.cache/api--pack: api--build per_account--output
	echo "Packing..."
	docker build lstc-api/. -t "lstc-api:${API_VERSION}" -t "${ECR_URL}:${API_VERSION}" && \
	touch $@

.cache/api--publish: .cache/api--pack per_account--output
	echo "Publishing..."
	aws ecr get-login-password --region eu-west-2 | docker login --username AWS --password-stdin ${ECR_URL}
	docker push "${ECR_URL}:${API_VERSION}" && \
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

api--pack: api--build .cache/api--pack									## Build the Docker container

api--publish: api--pack .cache/api--publish								## Publish the API to ECR
