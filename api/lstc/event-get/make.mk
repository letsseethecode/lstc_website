SRC_FILES ?= $(shell find api/lstc/event-get/src/ -name '*.rs')

api/lstc/event-get/target/lambda/event_get/bootstrap.zip: $(SRC_FILES)
	cd api/lstc/event-get; \
	cargo lambda build --release --output-format binary ; \
	mkdir -p out; \
	cp target/lambda/event-get/* out/

api/lstc/event-get/target/lambda: api/lstc/event-get/target/lambda/event_get/bootstrap.zip

api/lstc/event-get: api/lstc/event-get/target/lambda/event_get/bootstrap.zip
