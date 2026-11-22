use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aegis", version, about = "A local-first adaptive AI engineering agent")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show Aegis build and runtime identity.
    About,
    /// Validate a local Aegis TOML configuration.
    CheckConfig {
        #[arg(long, default_value = "config/default.toml")]
        config: PathBuf,
    },
    /// Inspect this machine and show a conservative resource profile.
    Profile,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::About => {
            println!("Aegis 0.1.0");
            println!("Local-first Rust AI engineering agent.");
            println!("Inference is designed to run on this machine; no hosted model API is required.");
            println!("Startup phrase: I summon you, Aegis.");
        }
        Commands::CheckConfig { config } => {
            match aegis_core::AegisConfig::load(&config) {
                Ok(settings) => {
                    println!("Configuration is valid.");
                    println!("Agent: {}", settings.agent.name);
                    println!("Backend: {}", settings.model.backend);
                    println!("Model: {}", settings.model.model_path);
                    println!("Adaptive mode: {}", settings.adaptive.enabled);
                    println!("Network access: {}", settings.runtime.allow_network);
                }
                Err(error) => {
                    eprintln!("Configuration error: {error}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Profile => {
            let profile = aegis_hardware::HardwareProfile::detect();
            println!("Aegis hardware profile");
            println!("OS / architecture: {} / {}", profile.operating_system, profile.architecture);
            println!("Logical CPU cores: {}", profile.logical_cores);
            print_memory("Total RAM", profile.total_memory_mb);
            print_memory("Available RAM", profile.available_memory_mb);
            println!("Resource tier: {}", profile.resource_tier.as_str());
            println!("Suggested context: {} tokens", profile.recommended_context_tokens);
            print_memory("Initial model memory budget", profile.model_memory_budget_mb);
            println!("Note: recommendations are estimates; actual model compatibility must be tested.");
        }
    }
}

fn print_memory(label: &str, value: Option<u64>) {
    match value {
        Some(mb) => println!("{label}: {mb} MiB"),
        None => println!("{label}: unavailable on this platform"),
    }
}
