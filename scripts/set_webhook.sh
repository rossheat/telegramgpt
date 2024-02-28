#!/bin/bash

WEBHOOK_URL=$(terraform -chdir=infra output -raw webhook_url)
TELEGRAM_BOT_TOKEN=$(grep telegram_bot_token ./infra/terraform.tfvars | cut -d '"' -f2)
TELEGRAM_WEBHOOK_SECRET_TOKEN=$(grep telegram_webhook_secret_token ./infra/terraform.tfvars | cut -d '"' -f2)

echo "Deleting current webhook and dropping pending updates..."
curl -s -X POST "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/deleteWebhook?drop_pending_updates=true"


echo "Registering new webhook URL with Telegram Bot API, including secret token, and dropping pending updates..."
curl -s -X POST \
    -H "Content-Type: application/json" \
    -d "{\"url\": \"$WEBHOOK_URL\", \"allowed_updates\": [\"message\"], \"drop_pending_updates\": true, \"secret_token\": \"$TELEGRAM_WEBHOOK_SECRET_TOKEN\"}" \
    "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/setWebhook"

echo "Fetching current webhook status..."
curl -s "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/getWebhookInfo"
