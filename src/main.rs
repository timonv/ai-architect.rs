#[macro_use]
extern crate pest_derive;

mod design_prompter;
mod parser;

use std::io::Write;

use design_prompter::DesignPrompter;

#[tokio::main]
async fn main() {
    // Do a couple things
    // Start a prompter loop
    // set_key();
    // let openapi

    let design_prompter = DesignPrompter::new(std::env::var("OPENAI_API_KEY").unwrap());

    loop {
        print!("User: ");
        std::io::stdout().flush().unwrap();

        let mut user_message_content = String::new();
        std::io::stdin()
            .read_line(&mut user_message_content)
            .unwrap();

        let returned_message = design_prompter
            .send_message(user_message_content)
            .await
            .unwrap();

        println!("{}", &returned_message)
    }

    // Generate 3dl from prompter loop
    // Visualize 3dl with ie mermaid
}
