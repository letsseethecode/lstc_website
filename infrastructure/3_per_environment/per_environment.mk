.PHONY: per_environment--clean per_environment--init per_environment--plan per_environment--apply per_environment--destroy
SRC_FILES = $(shell find ./infrastructure/3_per_environment -name "*.tf")
ENV ?= dev
ENV_PLAN := infrastructure/3_per_environment/per_environment__%.tfplan
ENV_OUTPUT := infrastructure/3_per_environment/output__%.json

per_environment--clean:
	rm infrastructure/3_per_environment/per_environment__*.tfplan

per_environment--init:
	cd infrastructure/3_per_environment && \
	terraform init \
		-var-file=./vars/${ENV}.tfvars

$(ENV_PLAN): $(SRC_FILES) $(BUILD_OUTPUT) $(WEB_ASSETS)
	cd ${dir $@} && \
	terraform workspace select -or-create=true ${*} && \
	terraform plan \
		-var-file=./vars/${*}.tfvars \
		-out=${notdir $@}

$(ENV_OUTPUT): $(ENV_PLAN)
	cd ${dir $@} && \
	terraform workspace select -or-create=true ${ENV} && \
	terraform apply \
		./per_environment__${*}.tfplan && \
	terraform output -json > ${notdir $@}

per_environment--plan: infrastructure/3_per_environment/per_environment__${ENV}.tfplan

per_environment--apply: infrastructure/3_per_environment/output__${ENV}.json

per_environment--destroy:
	cd infrastructure/3_per_environment && \
	terraform destroy \
		-var-file=./vars/${ENV}.tfvars
