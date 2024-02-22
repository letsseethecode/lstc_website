.PHONE 		:= web--clean web--build
WEB_OUTPUT	:= lstc-web/dist/index.html
WEB_ASSETS  := $(shell find lstc-web/dist)

$(WEB_OUTPUT):
	cd lstc-web && \
	trunk build --release

web--install:							## Install pre-requisites for lstc-web
	echo "Not implemented"

web--clean: 							## Delete the Website
	rm -rf lstc-web/target && \
	rm -rf lstc-web/dist && \
	rm -rf .cache/web--*

web--build: $(WEB_OUTPUT)	## Build the website

