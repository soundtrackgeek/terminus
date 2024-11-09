use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

mod llm;
use crate::llm::LLMClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The prompt to send to the LLM
    #[arg(short, long)]
    prompt: String,

    /// Which LLM to use (default: gpt-3.5-turbo)
    #[arg(short, long, default_value = "gpt-4o-mini")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let args = Args::parse();

    let client = llm::OpenAIClient::new(&api_key);
    let response = client.complete(&args.prompt).await?;

    println!("Response: {}", response);
    Ok(())
}
