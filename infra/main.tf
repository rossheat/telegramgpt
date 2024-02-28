provider "aws" {
  region = "us-east-1"  
}

resource "aws_lambda_function" "webhook_function" {
  function_name = "webhook"  
  runtime       = "provided.al2"
  handler       = "bootstrap"
  architectures = ["arm64"]
  timeout       = 180  
  filename      = "../target/lambda/TelegramGPT/bootstrap.zip"
  role = aws_iam_role.lambda_role.arn

  environment {
    variables = {
      TELEGRAM_BOT_TOKEN       = var.telegram_bot_token
      OPENAI_API_KEY           = var.openai_api_key
      MONGODB_URI              = var.mongodb_uri
      ALLOWED_CHAT_IDS            = jsonencode(var.allowed_chat_ids)
      BOT_NAME           = var.bot_name
      MESSAGE_HISTORY_LENGTH   = var.message_history_length
      OPENAI_MODEL_NAME        = var.openai_model_name
      SYSTEM_PROMPT            = var.system_prompt
      MAX_TOKENS = var.max_tokens
      TELEGRAM_WEBHOOK_SECRET_TOKEN = var.telegram_webhook_secret_token
      PRODUCTION = true
    }
  }
}

resource "aws_iam_role" "lambda_role" {
  name = "webhook_lambda_role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = "sts:AssumeRole",
        Effect = "Allow",
        Principal = {
          Service = "lambda.amazonaws.com"
        },
      },
    ],
  })
}

resource "aws_iam_role_policy" "lambda_policy" {
  name = "webhook_lambda_policy"
  role = aws_iam_role.lambda_role.id
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = ["logs:*", "cloudwatch:*"],
        Effect = "Allow",
        Resource = "*",
      },
    ],
  })
}

resource "aws_api_gateway_rest_api" "webhook_api" {
  name        = "WebhookAPI"
  description = "API Gateway for Webhook Lambda Function"
}

resource "aws_api_gateway_resource" "webhook_resource" {
  rest_api_id = aws_api_gateway_rest_api.webhook_api.id
  parent_id   = aws_api_gateway_rest_api.webhook_api.root_resource_id
  path_part   = "webhook"
}

resource "aws_api_gateway_method" "webhook_method" {
  rest_api_id   = aws_api_gateway_rest_api.webhook_api.id
  resource_id   = aws_api_gateway_resource.webhook_resource.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "webhook_integration" {
  rest_api_id = aws_api_gateway_rest_api.webhook_api.id
  resource_id = aws_api_gateway_resource.webhook_resource.id
  http_method = aws_api_gateway_method.webhook_method.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.webhook_function.invoke_arn
}

resource "aws_api_gateway_deployment" "webhook_deployment" {
  depends_on = [
    aws_api_gateway_integration.webhook_integration
  ]

  rest_api_id = aws_api_gateway_rest_api.webhook_api.id
  stage_name  = "prod"
}

resource "aws_lambda_permission" "webhook_permission" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.webhook_function.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.webhook_api.execution_arn}/*/*"
}

output "api_gateway_url" {
  value = "${aws_api_gateway_deployment.webhook_deployment.invoke_url}"
}

output "webhook_url" {
  value = "${aws_api_gateway_deployment.webhook_deployment.invoke_url}/webhook"
}
