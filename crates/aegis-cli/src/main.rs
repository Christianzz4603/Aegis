use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aegis", version, about = "Aegis AI engineering agent")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    About,
    CheckConfig {
        #[arg(long, default_value = "config/default.toml")]
        config: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::About => {
            println!("Aegis 0.1.0");
            println!("Rust-first, provider-independent AI engineering agent.");
            println!("Initial model: lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled");
            println!("Model inference and tool execution are not implemented yet.");
        }
        Commands::CheckConfig { config } => match aegis_core::AegisConfig::load(&config) {
            Ok(settings) => {
                println!("Configuration is valid.");
                println!("Agent: {}", settings.agent.name);
                println!("Provider: {}", settings.model.provider);
                println!("Model: {}", settings.model.model_id);
                println!("Endpoint: {}", settings.model.base_url);
                println!("Runtime: {}", settings.runtime.mode);
            }
            Err(error) => {
                eprintln!("Configuration error: {error}");
                std::process::exit(1);
            }
        },
    }
}
