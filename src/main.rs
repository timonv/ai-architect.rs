#[macro_use]
extern crate pest_derive;

mod parser;

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

use std::io::Write;

#[tokio::main]
async fn main() {
    // Do a couple things
    // Start a prompter loop
    set_key(std::env::var("OPENAI_API_KEY").unwrap());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: r#"
            You are a software system designer. Your role is to design a software system using the custom 3DL system design language.

            Here is an example of 3DL:
            ---
            package Test version 1.0.0 {
                    public entity TestEntity {
                        method get_name() -> string
                        method set_name(name: string)
                        method get_and_set_name(name: string) -> string
                        method get_multiple_parameters(name: string, age: integer) -> string
                    }
                }
            ---
            
            The 3DL you generate is parsed using the Rust package Pest. Here is the full grammar with comments of what you can use in the design:
            ---
            D3L = _{ SOI ~ package ~ EOI }
            package = { "package" ~ identifier ~ version* ~ entities* }
            entities = _{ ("{" ~ entity* ~ "}") }
            entity = { scope* ~ "entity" ~ identifier ~ ( "{" ~ attributes? ~ methods? ~ "}" )? }

            attributes = _{ "(" ~ attribute* ~ ")" }
            attribute = { identifier ~ ":" ~ atype ~ ","? }

            methods = _{ method+ }
            method = ${ "method " ~ identifier ~ parameters ~ returns? }

            scope = { "public" | "private" }
            atype = { identifier }
            parameters = !{ "(" ~ attribute* ~ ")" }
            returns = { " -> " ~ atype }

            identifier = @{ (ASCII_ALPHANUMERIC|"_")+ }
            version = { "version" ~ number ~ "." ~ number ~ "." ~ number }
            number = @{ ASCII_DIGIT+ }
            ---

            When processing more messages to update the design, ONLY change what the user requested. Do not change the design of the system unless the user explicitly requests it. Then always include the full design.

            Respond ONLY with a json object, and include no other commentary, in this format:
            ```
            {
              "3dl": "The 3DL",
              "mermaid": "The system design in valid mermaid format",
              "thoughts": "Any additional thoughts or comments you have on the design",
              
            }

            ```
        "#.to_string(),
        name: None
    }];

    loop {
        print!("User: ");
        std::io::stdout().flush().unwrap();

        let mut user_message_content = String::new();
        std::io::stdin()
            .read_line(&mut user_message_content)
            .unwrap();

        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: user_message_content,
            name: None,
        });

        let chat_completion = ChatCompletion::builder("gpt-4", messages.clone())
            .create()
            .await
            .unwrap();

        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.trim()
        )
    }

    // Generate 3dl from prompter loop
    // Visualize 3dl with ie mermaid
}
