use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

mod boot;
mod llm;
mod memory;
mod settings;
mod systemmessage;

use crate::memory::Memory;
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

    /// Add new memory entry
    #[arg(long)]
    add_memory: Option<String>,

    /// Show current memory
    #[arg(long)]
    show_memory: bool,

    /// Toggle memory usage
    #[arg(long)]
    toggle_memory: bool,

    /// Edit memory file in default text editor
    #[arg(long)]
    edit_memory: bool,
}

const AVAILABLE_MODELS: &[&str] = &["gpt-4o", "chatgpt-4o-latest", "gpt-4o-mini"];

async fn handle_menu_choice(choice: &str, settings: &mut Settings) -> Result<bool> {
    match choice {
        "1" => {
            print!("Enter your prompt: ");
            std::io::stdout().flush()?;
            let mut prompt = String::new();
            std::io::stdin().read_line(&mut prompt)?;

            let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
            let client = llm::OpenAIClient::new(&api_key, &settings.model);
            let system_message = SystemMessage::load()?;
            let memory_content = if settings.use_memory {
                Memory::load()?
            } else {
                String::new()
            };
            let memory = if settings.use_memory {
                Some(memory_content.as_str())
            } else {
                None
            };
            let response = client
                .complete_with_system(&prompt, &system_message, memory)
                .await?;
            println!("Response: {}", response);
        }
        "2" => {
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
            } else {
                println!("Invalid selection");
            }
        }
        "3" => {
            print!("Enter new system message: ");
            std::io::stdout().flush()?;
            let mut message = String::new();
            std::io::stdin().read_line(&mut message)?;
            SystemMessage::save(&message)?;
            println!("System message updated successfully");
        }
        "4" => {
            let message = SystemMessage::load()?;
            println!("Current system message:\n{}", message);
        }
        "5" => {
            print!("Enter memory entry: ");
            std::io::stdout().flush()?;
            let mut entry = String::new();
            std::io::stdin().read_line(&mut entry)?;
            Memory::append(&entry)?;
            println!("Memory entry added successfully");
        }
        "6" => {
            let memory = Memory::load()?;
            println!("Current memory:\n{}", memory);
        }
        "7" => {
            settings.use_memory = !settings.use_memory;
            settings.save()?;
            println!(
                "Memory usage: {}",
                if settings.use_memory {
                    "enabled"
                } else {
                    "disabled"
                }
            );
        }
        "8" => {
            Memory::edit()?;
        }
        "9" => return Ok(true),
        _ => println!("Invalid choice, please try again"),
    }
    Ok(false)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();
    let mut settings = Settings::load()?;

    // If no arguments provided, run interactive mode
    if args.prompt.is_none()
        && !args.select_model
        && args.set_system.is_none()
        && !args.show_system
        && args.add_memory.is_none()
        && !args.show_memory
        && !args.toggle_memory
        && !args.edit_memory
    {
        boot::boot_sequence();

        loop {
            let choice = boot::show_menu()?;
            if handle_menu_choice(&choice, &mut settings).await? {
                break;
            }
        }
        return Ok(());
    }

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

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

    if args.show_memory {
        let memory = Memory::load()?;
        println!("Current memory:\n{}", memory);
        return Ok(());
    }

    if let Some(memory_entry) = args.add_memory {
        Memory::append(&memory_entry)?;
        println!("Memory entry added successfully");
        return Ok(());
    }

    if args.toggle_memory {
        settings.use_memory = !settings.use_memory;
        settings.save()?;
        println!(
            "Memory usage: {}",
            if settings.use_memory {
                "enabled"
            } else {
                "disabled"
            }
        );
        return Ok(());
    }

    if args.edit_memory {
        Memory::edit()?;
        return Ok(());
    }

    if let Some(prompt) = args.prompt {
        let client = llm::OpenAIClient::new(&api_key, &settings.model);
        let system_message = SystemMessage::load()?;
        let memory_content = if settings.use_memory {
            Memory::load()?
        } else {
            String::new()
        };
        let memory = if settings.use_memory {
            Some(memory_content.as_str())
        } else {
            None
        };
        let response = client
            .complete_with_system(&prompt, &system_message, memory)
            .await?;
        println!("Response: {}", response);
    } else {
        println!("Please provide a prompt with --prompt or select a model with --select-model");
    }

    Ok(())
}
