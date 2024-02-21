.PHONY := per_account--clean per_account--init per_account--plan per_account--apply per_account--destroy
SRC_FILES = $(shell find ./infrastructure/2_per_account -name "*.tf")

per_account--clean:
	rm infrastructure/2_per_account/per_account.tfplan

per_account--init:
	cd infrastructure/2_per_account && \
	terraform init \
		-var-file=./vars/${ACCOUNT}.tfvars

infrastructure/2_per_account/per_account.tfplan: $(SRC_FILES)
	cd infrastructure/2_per_account && \
	terraform workspace select -or-create=true ${ACCOUNT} && \
	terraform plan \
		-var-file=./vars/${ACCOUNT}.tfvars \
		-out=./per_account.tfplan

per_account--plan: infrastructure/2_per_account/per_account.tfplan

per_account--apply: per_account--plan
	cd infrastructure/2_per_account && \
	terraform workspace select -or-create=true ${ACCOUNT} && \
	terraform apply \
		./per_account.tfplan

per_account--destroy:
	cd infrastructure/2_per_account && \
	terraform destroy \
		-var-file=./vars/${ACCOUNT}.tfvars
