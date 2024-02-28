#!/bin/bash

TELEGRAM_WEBHOOK_SECRET_TOKEN=$(grep telegram_webhook_secret_token ./infra/terraform.tfvars | cut -d '"' -f2)
WEBHOOK_URL='http://127.0.0.1:9000/lambda-url/telegramgpt/'
JSON_DATA=$(jq -r '.body' ./test_data/update.json)

curl -H "Content-Type: application/json" \
     -H "X-Telegram-Bot-Api-Secret-Token: $TELEGRAM_WEBHOOK_SECRET_TOKEN" \
     -d "$JSON_DATA" \
     "$WEBHOOK_URL"
