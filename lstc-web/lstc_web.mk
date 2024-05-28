.PHONE 		:= web--clean web--build
WEB_OUTPUT	:= lstc-web/dist/index.html
WEB_ASSETS  := $(shell find lstc-web/dist || true)
SRC_FILES	:= $(shell find lstc-web/src -name "*.rs")
WEB_ECR_URL := $(shell cat infrastructure/2_per_account/output.json | jq -r .ecr_web_url.value)
WEB_VERSION	?= $(shell grep -E "^version" lstc-web/Cargo.toml | grep -Eo "[0-9+]\.[0-9+]\.[0-9+]")

$(WEB_OUTPUT): $(SRC_FILES)
	cd lstc-web && \
	trunk build --release

.cache/web--pack__%: infrastructure/2_per_account/output.json $(WEB_OUTPUT)
	docker build lstc-web/. -t "lstc-web:${*}" -t "lstc-web:latest" -t "${WEB_ECR_URL}:${*}"
	touch $@

.cache/web--publish__%: .cache/web--pack__%
	aws ecr get-login-password --region eu-west-2 | docker login --username AWS --password-stdin ${WEB_ECR_URL}
	docker push "${WEB_ECR_URL}:${*}"
	touch $@

web--install:							## Install pre-requisites for lstc-web
	cargo install cargo-binstall
	cargo binstall --locked trunk

web--clean: 							## Delete the Website
	rm -rf lstc-web/target || true && \
	rm -rf lstc-web/dist || true && \
	rm -rf .cache/web--* || true

web--build: $(WEB_OUTPUT)									## Compile the website

web--pack: .cache/web--pack__${WEB_VERSION}					## Build the Docker container

web--publish: .cache/web--publish__${WEB_VERSION}			## Publish the WEB to ECR
