use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Prompt {
    prompt: String,
    max_tokens: i32,
}

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
}

struct Client {
    api_url: String,
}

impl Client {
    async fn request(&self, prompt: &Prompt) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let resp = client
            .post(&self.api_url)
            .json(prompt)
            .send()
            .await?
            .text()
            .await;

        dbg!(&resp);
        let resp = client
            .post(&self.api_url)
            .json(prompt)
            .send()
            .await?
            .json::<Response>()
            .await;

        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mocked_client_with_response(response: &str) -> Client {
        let mut server = mockito::Server::new();
        server
            .mock("POST", mockito::Matcher::Any)
            .with_body(response)
            .create();

        Client {
            api_url: server.url(),
        }
    }

    #[tokio::test]
    async fn test_request() {
        // Create a dummy prompt for testing
        let prompt = Prompt {
            prompt: String::from("Test prompt"),
            max_tokens: 10,
        };

        // Create a mock HTTP response
        let response_body = r#"{
            "id": "response_id",
            "object": "response_object",
            "created": 1622457600,
            "model": "response_model",
            "choices": [
                {
                    "text": "Choice 1"
                },
                {
                    "text": "Choice 2"
                }
            ]
        }"#;

        let result = mocked_client_with_response(response_body)
            .request(&prompt)
            .await;
        dbg!(&result);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, "response_id");
        assert_eq!(response.object, "response_object");
        assert_eq!(response.created, 1622457600);
        assert_eq!(response.model, "response_model");
        assert_eq!(response.choices.len(), 2);
        assert_eq!(response.choices[0].text, "Choice 1");
        assert_eq!(response.choices[1].text, "Choice 2");
    }

    #[tokio::test]
    async fn test_invalid_response_usable_error() {
        // TODO: Ideally return bad errors to user or something
        let prompt = Prompt {
            prompt: String::from("Test prompt"),
            max_tokens: 10,
        };

        // Create a mock HTTP response
        let response_body = r#"Invalid json"#;

        let result = mocked_client_with_response(response_body)
            .request(&prompt)
            .await;
        dbg!(&result);
        assert!(result.is_err());
    }
}
