.PHONY := help
.DEFAULT_GOAL := help

MAKE_FILES := $(shell find . -name "*.mk")

include $(MAKE_FILES)

help: ## This help message
	@echo "Available targets:"
	@grep -E --no-filename '^[a-zA-Z-]+:.*?## .*$$' Makefile $(MAKE_FILES) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-42s\033[0m %s\n", $$1, $$2}'
