#!/bin/bash

TELEGRAM_BOT_TOKEN=$(grep telegram_bot_token ./infra/terraform.tfvars | cut -d '"' -f2)
GET_UPDATES_URL="https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/getUpdates"
RESPONSE=$(curl -s -X POST $GET_UPDATES_URL -d "limit=1" -d "allowed_updates=[\"message\"]")

echo $RESPONSE
