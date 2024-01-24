#!/bin/bash
#
# This file is executed once during the entire lifespan of the
# project, and should only be re-run on new accounts or when new
# resources are required that cannot be implemented in terraform.

if [[ "$AWS_PROFILE" = "" ]]; then
    echo 
    echo "ERROR: Please ensure that you have specified an AWS_PROFILE before running this command."
    echo 
    exit 1
fi

TERRAFORM_STATE=lstc-terraform-state2
TERRAFORM_LOCK=lstc-terraform-lock
TERRAFORM_REGION=eu-west-2

aws s3api get-bucket-location \
    --bucket $TERRAFORM_STATE \
    --region $TERRAFORM_REGION \
    2>&1 > /dev/null

if [[ $? != 0 ]]; then
    echo "creating terraform state bucket..."
    aws s3api create-bucket \
	--bucket $TERRAFORM_STATE \
	--region $TERRAFORM_REGION \
	--create-bucket-configuration LocationConstraint="${TERRAFORM_REGION}"
else
    echo "terraform state exists..."
fi

aws dynamodb describe-table \
    --table-name $TERRAFORM_LOCK \
    --region $TERRAFORM_REGION \
    2>&1 > /dev/null

if [[ $? != 0 ]]; then
    echo "creating lock..."
    aws dynamodb create-table \
	--table-name $TERRAFORM_LOCK \
	--region $TERRAFORM_REGION \
	--attribute-definitions '[{ "AttributeName": "LockID", "AttributeType": "S" }]' \
	--key-schema '[{ "AttributeName": "LockID", "KeyType": "HASH" }]' \
	--billing-mode PAY_PER_REQUEST
else
    echo "terraform lock exists..."
fi

echo "Done"
