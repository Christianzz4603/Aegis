use clap::{Parser, Subcommand};
use std::{env, fs, io::{self, BufRead, Write}, path::{Path, PathBuf}};
const DEFAULT_CONFIG: &str = include_str!("../../../config/default.toml");
#[derive(Parser)]
#[command(name = "aegis", version, about = "A local-first adaptive AI engineering agent")]
struct Cli { #[command(subcommand)] command: Option<Commands> }
#[derive(Subcommand)]
enum Commands { About, Setup, Chat, CheckConfig { #[arg(long)] config: Option<PathBuf> }, Profile }
struct Paths { config_dir: PathBuf, data_dir: PathBuf, cache_dir: PathBuf, config_file: PathBuf, model_dir: PathBuf }
fn main() {
 let cli = Cli::parse();
 let result = match cli.command {
  Some(Commands::About) => { println!("Aegis {} - local-first Rust AI agent", env!("CARGO_PKG_VERSION")); Ok(()) },
  Some(Commands::Setup) => setup(true).map(|_| ()),
  Some(Commands::Chat) => setup(false).and_then(|p| interactive(&p)),
  Some(Commands::CheckConfig { config }) => check_config(&config.unwrap_or_else(|| paths().config_file)),
  Some(Commands::Profile) => { profile(); Ok(()) },
  None => setup(false).and_then(|p| interactive(&p)),
 };
 if let Err(e) = result { eprintln!("Aegis: {e}"); std::process::exit(1); }
}
fn paths() -> Paths {
 let home = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")).map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
 let cb = env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).unwrap_or_else(|| if cfg!(windows) { env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| home.join("AppData/Roaming")) } else { home.join(".config") });
 let db = env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|| if cfg!(windows) { env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(|| home.join("AppData/Local")) } else { home.join(".local/share") });
 let kb = env::var_os("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or_else(|| home.join(".cache"));
 let config_dir=cb.join("aegis"); let data_dir=db.join("aegis"); let cache_dir=kb.join("aegis");
 let model_dir=data_dir.join("models"); let config_file=config_dir.join("config.toml");
 Paths { config_dir, data_dir, cache_dir, config_file, model_dir }
}
fn setup(show_existing: bool) -> io::Result<Paths> {
 let p=paths(); let first=!p.config_file.exists();
 fs::create_dir_all(&p.config_dir)?; fs::create_dir_all(&p.data_dir)?; fs::create_dir_all(&p.cache_dir)?; fs::create_dir_all(&p.model_dir)?;
 if first {
  let model=p.model_dir.join("aegis-model.gguf").to_string_lossy().replace('\\',"\\\\").replace('"',"\\\"");
  fs::write(&p.config_file,DEFAULT_CONFIG.replace("models/aegis-model.gguf",&model))?;
  println!("Welcome to Aegis. First-run setup completed.");
  println!("Config: {}\nModels: {}\nData: {}\nCache: {}",p.config_file.display(),p.model_dir.display(),p.data_dir.display(),p.cache_dir.display());
  println!("No model was downloaded. Large model downloads always require your consent.");
  println!("Note: local inference is not connected in this early build yet.");
 } else if show_existing { println!("Aegis is already initialized.\nConfig: {}\nModels: {}",p.config_file.display(),p.model_dir.display()); }
 Ok(p)
}
fn check_config(path:&Path)->Result<(),Box<dyn std::error::Error>> {
 let c=aegis_core::AegisConfig::load(path)?;
 println!("Configuration is valid. Agent: {} | backend: {} | model: {}",c.agent.name,c.model.backend,c.model.model_path); Ok(())
}
fn profile() {
 let p=aegis_hardware::HardwareProfile::detect();
 println!("Aegis hardware profile\nOS / architecture: {} / {}\nLogical CPU cores: {}",p.operating_system,p.architecture,p.logical_cores);
 print_memory("Total RAM",p.total_memory_mb); print_memory("Available RAM",p.available_memory_mb);
 println!("Resource tier: {}\nSuggested context: {} tokens",p.resource_tier.as_str(),p.recommended_context_tokens);
 print_memory("Initial model memory budget",p.model_memory_budget_mb);
}
fn print_memory(label:&str,value:Option<u64>) { match value { Some(v)=>println!("{label}: {v} MiB"),None=>println!("{label}: unavailable") } }
fn interactive(p:&Paths)->io::Result<()> {
 println!("\nA E G I S | Local terminal session");
 println!("Type /help for commands. Working directory: {}",env::current_dir()?.display());
 let stdin=io::stdin(); let mut input=stdin.lock(); let mut line=String::new();
 loop {
  print!("You: "); io::stdout().flush()?; line.clear();
  if input.read_line(&mut line)?==0 { println!(); break; }
  let message=line.trim(); if message.is_empty() { continue; }
  match message {
   "/exit"|"/quit"=>break,
   "/help"=>println!("Commands: /help, /profile, /model, /setup, /exit. Ordinary prompts will be handled by the local model after an inference backend is integrated."),
   "/profile"=>profile(),
   "/setup"=>println!("Config: {}\nModels: {}\nData: {}\nCache: {}",p.config_file.display(),p.model_dir.display(),p.data_dir.display(),p.cache_dir.display()),
   "/model"=>match aegis_core::AegisConfig::load(&p.config_file) { Ok(c)=>{let m=PathBuf::from(&c.model.model_path);println!("Backend: {}\nModel: {}\nModel file: {}\nInference engine: not connected yet",c.model.backend,m.display(),if m.is_file(){"found"}else{"not installed"});},Err(e)=>println!("Could not read model configuration: {e}") },
   _=>{println!("Aegis: The terminal is ready, but a local language model is not connected in this build.");println!("Your message was not sent to a server or saved to a file. No model response was generated.");println!("Use /help for terminal commands.");}
  }
  println!();
 }
 println!("Aegis session ended."); Ok(())
}
