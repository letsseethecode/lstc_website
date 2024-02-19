.PHONE := web--clean web--build

lstc-web/dist/index.html:
	cd lstc-web && \
	trunk build --release

web--install:
	echo "Not implemented"

web--clean: ## Delete the Website
	rm -rf src/lstc-web/target && \
	rm -rf src/lstc-web/dist

web--build: lstc-web/dist/index.html ## Build the website

