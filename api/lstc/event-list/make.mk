SRC_FILES ?= $(shell find $(api/lstc/event-list)/src/ -name '*.rs')

api/lstc/event-list/target/lambda/event_list/bootstrap.zip: $(SRC_FILES)
	cd api/lstc/event-list; \
	cargo lambda build --release --output-format binary ; \
	mkdir -p out; \
	cp target/lambda/event-list/* out/

api/lstc/event-list/target/lambda: api/lstc/event-list/target/lambda/event_list/bootstrap.zip

api/lstc/event-list: api/lstc/event-list/target/lambda/event_list/bootstrap.zip
