use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

mod llm;
mod settings;
mod systemmessage;

use crate::llm::LLMClient;
use crate::settings::Settings;
use crate::systemmessage::SystemMessage;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The prompt to send to the LLM
    #[arg(short, long)]
    prompt: Option<String>,

    /// Select model to use
    #[arg(short, long)]
    select_model: bool,

    /// Set system message
    #[arg(long)]
    set_system: Option<String>,

    /// Show current system message
    #[arg(long)]
    show_system: bool,
}

const AVAILABLE_MODELS: &[&str] = &["gpt-4o", "chatgpt-4o-latest", "gpt-4o-mini"];

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let args = Args::parse();
    let mut settings = Settings::load()?;

    if let Some(message) = args.set_system {
        SystemMessage::save(&message)?;
        println!("System message updated successfully");
        return Ok(());
    }

    if args.show_system {
        let message = SystemMessage::load()?;
        println!("Current system message:\n{}", message);
        return Ok(());
    }

    if args.select_model {
        println!("Available models:");
        for (i, model) in AVAILABLE_MODELS.iter().enumerate() {
            println!("{}. {}", i + 1, model);
        }
        println!("Enter number (1-{}): ", AVAILABLE_MODELS.len());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let selection: usize = input.trim().parse()?;

        if selection > 0 && selection <= AVAILABLE_MODELS.len() {
            settings.model = AVAILABLE_MODELS[selection - 1].to_string();
            settings.save()?;
            println!("Model set to: {}", settings.model);
            return Ok(());
        } else {
            println!("Invalid selection");
            return Ok(());
        }
    }

    if let Some(prompt) = args.prompt {
        let client = llm::OpenAIClient::new(&api_key, &settings.model);
        let system_message = SystemMessage::load()?;
        let response = client
            .complete_with_system(&prompt, &system_message)
            .await?;
        println!("Response: {}", response);
    } else {
        println!("Please provide a prompt with --prompt or select a model with --select-model");
    }

    Ok(())
}
