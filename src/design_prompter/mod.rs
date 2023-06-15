use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use std::cell::RefCell;
use std::error;

mod genesis_prompt;
use genesis_prompt::GENESIS_PROMPT;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct DesignPrompter {
    messages: RefCell<Vec<ChatCompletionMessage>>,
}

impl DesignPrompter {
    pub fn new(openai_api_key: String) -> Self {
        set_key(openai_api_key);

        DesignPrompter {
            messages: RefCell::new(vec![ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: GENESIS_PROMPT.to_string(),
                name: None,
            }]),
        }
    }

    pub async fn send_message(&self, message: String) -> Result<String> {
        self.messages.borrow_mut().push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: message,
            name: None,
        });

        let cloned_messages = self.messages.borrow().clone();
        let chat_completion = ChatCompletion::builder("gpt-4", cloned_messages)
            .create()
            .await?;

        let response_message = &chat_completion
            .choices
            .first()
            .ok_or("No completion choices form openai")?
            .message;

        self.messages.borrow_mut().push(response_message.clone());

        Ok(response_message.content.to_string())
    }
}
