LAMBDA_DIRS := $(dir $(wildcard api/lstc/*/.))
API_OUTPUT := $(foreach dir,$(LAMBDA_DIRS),$(dir)target/lambda)

api--clean:
	rm -rf api/lstc/*/out || true
	rm -rf api/lstc/*/target || true

api--build: $(API_OUTPUT)
