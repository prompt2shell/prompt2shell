use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

pub async fn create_client(api_key: String) -> Client {
    let client = Client::new(api_key);
    client
}

pub async fn generate_command(
    client: &Client,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = ChatCompletionRequest {
        model: chat_completion::GPT4.to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: prompt.to_string(),
        }],
    };

    let result = client.chat_completion(req).await?;

    let command = &result.choices[0].message.content;

    // Delete \n from command (I don't know yet why the api put \n on the beginning of answers)
    let command = command.trim_start_matches('\n').to_string();

    Ok(command)
}
