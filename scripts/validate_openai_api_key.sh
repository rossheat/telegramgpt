#!/bin/bash

API_KEY=$(grep 'openai_api_key' infra/terraform.tfvars | cut -d '"' -f 2)

if [ -z "$API_KEY" ]; then
    echo "API Key not found in terraform.tfvars"
    exit 1
fi

REQUEST_BODY='{"input": "Test message"}'

RESPONSE_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "https://api.openai.com/v1/moderations" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $API_KEY" \
    -d "$REQUEST_BODY")

if [ "$RESPONSE_CODE" = "401" ]; then
    echo "Unauthorized: Invalid OpenAI API key. Please check your API key."
    exit 1
fi

echo "API key is valid. Continuing with deployment."
