.PHONE := web--clean web--build

lstc-web/dist/index.html:
	cd lstc-web && \
	trunk build --release

web--install:							## Install pre-requisites for lstc-web
	echo "Not implemented"

web--clean: 							## Delete the Website
	rm -rf lstc-web/target && \
	rm -rf lstc-web/dist && \
	rm -rf .cache/web--*

web--build: lstc-web/dist/index.html	## Build the website

