.PHONY := per_environment--clean per_environment--init per_environment--plan per_environment--apply per_environment--destroy
SRC_FILES = $(shell find ./infrastructure/3_per_environment -name "*.tf")

per_environment--clean:
	rm infrastructure/3_per_environment/per_environment.tfplan

per_environment--init:
	cd infrastructure/3_per_environment && \
	terraform init \
		-var-file=./vars/${ENV}.tfvars

infrastructure/3_per_environment/per_environment.tfplan: $(SRC_FILES) build
	cd infrastructure/3_per_environment && \
	terraform workspace select -or-create=true ${ENV} && \
	terraform plan \
		-var-file=./vars/${ENV}.tfvars \
		-out=./per_environment.tfplan

per_environment--plan: infrastructure/3_per_environment/per_environment.tfplan

per_environment--apply: per_environment--plan
	cd infrastructure/3_per_environment && \
	terraform workspace select -or-create=true ${ENV} && \
	terraform apply \
		./per_environment.tfplan

per_environment--destroy:
	cd infrastructure/3_per_environment && \
	terraform destroy \
		-var-file=./vars/${ENV}.tfvars
