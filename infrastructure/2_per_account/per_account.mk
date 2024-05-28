.PHONY: per_account--clean per_account--init per_account--plan per_account--apply per_account--destroy per_account--output
SRC_FILES = $(shell find ./infrastructure/2_per_account -name "*.tf")
ACCOUNT ?= default
DOMAIN ?= "letsseethecode.com"

per_account--clean:
	rm infrastructure/2_per_account/per_account.tfplan || true

per_account--init:
	cd infrastructure/2_per_account && \
	terraform init \
		-var-file=./vars/${ACCOUNT}.tfvars

infrastructure/2_per_account/per_account.tfplan: $(SRC_FILES)
	cd infrastructure/2_per_account && \
	terraform workspace select -or-create=true ${ACCOUNT} && \
	terraform plan \
		-var-file=./vars/${ACCOUNT}.tfvars \
		-var "domain=${DOMAIN}" \
		-out=./per_account.tfplan

infrastructure/2_per_account/output.json: infrastructure/2_per_account/per_account.tfplan
	cd infrastructure/2_per_account && \
	terraform workspace select -or-create=true ${ACCOUNT} && \
	terraform apply \
		./per_account.tfplan && \
	terraform output -json > ./output.json

per_account--plan: infrastructure/2_per_account/per_account.tfplan

per_account--apply: infrastructure/2_per_account/output.json

per_account--output:
	cd infrastructure/2_per_account && \
	terraform workspace select -or-create=true ${ACCOUNT} && \
	terraform apply -refresh-only && \
	terraform output -json > ./output.json

per_account--destroy:
	cd infrastructure/2_per_account && \
	terraform destroy \
		-var-file=./vars/${ACCOUNT}.tfvars
