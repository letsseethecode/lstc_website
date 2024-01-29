.PHONY := export_%

# this is a guard target, that causes the makefile to exit if an      │
# environment variable is not set.                                    │

expect_%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1
	fi
