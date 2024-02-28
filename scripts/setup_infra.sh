#!/bin/bash

echo "Initializing Terraform..."
terraform -chdir=infra init
echo "Destroying existing infrastructure..."
terraform -chdir=infra destroy --auto-approve
echo "Applying new infrastructure..."
terraform -chdir=infra apply --auto-approve
