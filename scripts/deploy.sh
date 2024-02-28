#!/bin/bash

echo "Validating OpenAI API key using validate_openai_api_key..."
./scripts/validate_openai_api_key.sh
if [ $? -ne 0 ]; then
    echo "OpenAI API key validation failed. Exiting deployment."
    exit 1
fi

if [ "$1" = "--delete-messages" ]; then
    echo "Deleting messages using delete_messages.sh..."
    ./scripts/delete_messages.sh
fi

echo "Building Lambda package using build_package.sh..."
./scripts/build_package.sh 

echo "Setting up infrastructure using setup_infra.sh..."
./scripts/setup_infra.sh

echo "Registering Webhook URL with Telegram Bot API using set_webhook.sh..."
./scripts/set_webhook.sh

echo "Testing the Webhook URL using prod_test.sh..."
./scripts/prod_test.sh

echo "Deployment and testing completed."
