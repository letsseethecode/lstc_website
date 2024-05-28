.PHONY: api api--install api--clean api--build api--pack api--publish

SRC_FILES 	:= $(shell find ./lstc-api -name "*.rs")
API_VERSION	?= $(shell grep -E "^version" lstc-api/Cargo.toml | grep -Eo "[0-9+]\.[0-9+]\.[0-9+]")
ECR_URL 	:= $(shell cat infrastructure/2_per_account/output.json | jq -r .ecr_api_url.value)
API_OUTPUT 	:= lstc-api/target/release/lstc-api

include utils.mk

#
# These are the proper targets for building and publishing the API
#

$(API_OUTPUT): $(SRC_FILES)
	cd lstc-api && \
	cargo build --release

.cache/api--pack__%: infrastructure/2_per_account/output.json $(API_OUTPUT)
	docker build lstc-api/. -t "lstc-api:${*}" -t "${ECR_URL}:${*}"
	touch $@

.cache/api--publish__%: .cache/api--pack__%
	aws ecr get-login-password --region eu-west-2 | docker login --username AWS --password-stdin ${ECR_URL}
	docker push "${ECR_URL}:${*}"
	touch $@

#
# These are the short-hand targets for the api, which are easier to use
#

api--install:												## Install pre-requisites for lstc-api
	echo "Not implemented"

api--clean:													## Clean up the API files
	rm -rf lstc-api/target/* || true
	rm -f .cache/api--* || true

api--build: $(API_OUTPUT)									## Compile the API

api--pack: .cache/api--pack__${API_VERSION}					## Build the Docker container

api--publish: .cache/api--publish__${API_VERSION}			## Publish the API to ECR
