#!/bin/bash

TELEGRAM_WEBHOOK_SECRET_TOKEN=$(grep telegram_webhook_secret_token ./infra/terraform.tfvars | cut -d '"' -f2)
WEBHOOK_URL=$(terraform -chdir=infra output -raw webhook_url)
JSON_DATA=$(jq -r '.body' ./test_data/update.json)

echo "JSON data being sent:"
echo "$JSON_DATA"

curl -H "Content-Type: application/json" \
     -H "X-Telegram-Bot-Api-Secret-Token: $TELEGRAM_WEBHOOK_SECRET_TOKEN" \
     -d "$JSON_DATA" \
     "$WEBHOOK_URL"
