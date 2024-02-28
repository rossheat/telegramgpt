variable "telegram_bot_token" {
  description = "The token used to authenticate the Telegram bot."
  type        = string
}

variable "openai_api_key" {
  description = "API key for accessing OpenAI services."
  type        = string
}

variable "mongodb_uri" {
  description = "Connection URI for the MongoDB database."
  type        = string
}

variable "allowed_chat_ids" {
  description = "List of chat IDs authorized to interact with the bot."
  type        = list(number)
}

variable "bot_name" {
  description = "The name assigned to the Telegram bot."
  type        = string
}

variable "message_history_length" {
  description = "The number of previous chat messages that get sent to the chat model."
  type        = number
}

variable "openai_model_name" {
  description = "The name of the OpenAI model to be used."
  type        = string
}

variable "system_prompt" {
  description = "Default system prompt for initiating conversations."
  type        = string
}

variable "production" {
  description = "Flag to indicate whether the environment is Production or Development."
  type        = bool
}

variable "max_tokens" {
  description = "Maximum number of tokens to be used in processing requests."
  type        = number
}

variable "telegram_webhook_secret_token" {
  description = "Secret token for securing the Telegram webhook."
  type        = string 
}
