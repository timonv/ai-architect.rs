use async_trait::async_trait;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Serialize)]
struct Prompt {
    prompt: String,
    max_tokens: i32,
}

#[derive(Deserialize)]
struct Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[async_trait]
pub trait Client {
    async fn request(&self, prompt: &Prompt) -> Result<Response, Error>;
}

pub struct GptClient;

#[async_trait]
impl Client for GptClient {
    async fn request(&self, prompt: &Prompt) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.openai.com/v1/engines/davinci/completions")
            .json(prompt)
            .send()
            .await?
            .json::<Response>()
            .await;

        resp
    }
}

pub struct ChatService {
    client: Arc<dyn Client>,
}

impl ChatService {
    pub fn new(client: Arc<dyn Client>) -> Self {
        Self { client }
    }

    pub fn chat(&self, prompt: &Prompt) -> Result<Response, Error> {
        self.client.request(prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    struct MockClient;

    impl Client for MockClient {
        fn request(&self, _prompt: &Prompt) -> Result<Response, Error> {
            let res = Response {
                id: "test_id".to_string(),
                object: "text.completion".to_string(),
                created: 0,
                model: "gpt-4.0-turbo".to_string(),
                choices: vec![Choice {
                    text: "Translated text".to_string(),
                }],
            };

            Ok(res)
        }
    }

    #[test]
    fn test_chat_service() {
        let chat_service = ChatService::new(Arc::new(MockClient));

        let prompt = Prompt {
            prompt: "Translate the following English text to French: '{}'",
            max_tokens: 60,
        };

        let res = chat_service.chat(&prompt).unwrap();

        assert_eq!(res.id, "test_id");
        assert_eq!(res.choices[0].text, "Translated text");
    }
}
