.PHONY=api api--build api--pack api--publish expect_%
.DEFAULT=api--build

SRC_FILES:=./lstc_api/src/main.rs
# GIT_HASH ?= $(shell git log --format="%h" -n 1)

expect_%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1; \
	fi

lstc_api/target/release/lstc_api: $(SRC_FILES)
	cd lstc_api && \
	cargo build --release

api--build: lstc_api/target/release/lstc_api

.cache/api--pack__%: expect_ECR api--build
	echo "Packing..."
	docker build lstc_api/. -t "lstc_api:${*}" -t "${ECR}/lstc_api:${*}" && \
	touch $@

.cache/api--publish__%: expect_ECR .cache/api--pack__%
	echo "Publishing..."
	docker push "${ECR}/lstc_api:${*}" && \
	touch $@

api--pack: expect_VERSION .cache/api--pack__$(VERSION)

api--publish: expect_VERSION .cache/api--publish__$(VERSION)

api--clean:
	rm -rf lstc_api/target/*
	rm -f .cache/api--*
