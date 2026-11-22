use clap::{Parser, Subcommand};
use crossterm::style::{Color, Stylize};
use std::{
    env, fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

const DEFAULT_CONFIG: &str = include_str!("../../../config/default.toml");

#[derive(Parser)]
#[command(name = "aegis", version, about = "A local-first adaptive AI engineering agent")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    About,
    Setup,
    Chat,
    CheckConfig {
        #[arg(long)]
        config: Option<PathBuf>,
    },
    Profile,
}

struct Paths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    config_file: PathBuf,
    model_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Some(Commands::About) => {
            println!("Aegis {} | local-first Rust AI agent", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some(Commands::Setup) => setup(true).map(|_| ()),
        Some(Commands::Chat) => setup(false).and_then(|p| interactive(&p)),
        Some(Commands::CheckConfig { config }) => {
            check_config(&config.unwrap_or_else(|| paths().config_file))
        }
        Some(Commands::Profile) => {
            profile();
            Ok(())
        }
        None => setup(false).and_then(|p| interactive(&p)),
    };

    if let Err(error) = result {
        eprintln!("{} {error}", paint("✖", Color::Red));
        std::process::exit(1);
    }
}

fn paint(text: &str, color: Color) -> String {
    if env::var_os("NO_COLOR").is_some() || env::var("CLICOLOR").as_deref() == Ok("0") {
        text.to_owned()
    } else {
        format!("{}", text.with(color))
    }
}

fn blue(text: &str) -> String {
    paint(text, Color::Rgb { r: 75, g: 155, b: 255 })
}

fn muted(text: &str) -> String {
    paint(text, Color::DarkGrey)
}

fn paths() -> Paths {
    let home = env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let config_base = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if cfg!(windows) {
                env::var_os("APPDATA")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| home.join("AppData/Roaming"))
            } else {
                home.join(".config")
            }
        });
    let data_base = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if cfg!(windows) {
                env::var_os("LOCALAPPDATA")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| home.join("AppData/Local"))
            } else {
                home.join(".local/share")
            }
        });
    let cache_base = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".cache"));
    let config_dir = config_base.join("aegis");
    let data_dir = data_base.join("aegis");
    let cache_dir = cache_base.join("aegis");
    let model_dir = data_dir.join("models");
    let config_file = config_dir.join("config.toml");
    Paths {
        config_dir,
        data_dir,
        cache_dir,
        config_file,
        model_dir,
    }
}

fn spinner(label: &str, duration: Duration) {
    let active = Arc::new(AtomicBool::new(true));
    let thread_active = Arc::clone(&active);
    let label = label.to_owned();
    let thread_label = label.clone();
    let worker = thread::spawn(move || {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut index = 0;
        while thread_active.load(Ordering::Relaxed) {
            print!("\r{} {}", blue(frames[index % frames.len()].to_owned().as_str()), muted(&thread_label));
            let _ = io::stdout().flush();
            index += 1;
            thread::sleep(Duration::from_millis(75));
        }
    });
    thread::sleep(duration);
    active.store(false, Ordering::Relaxed);
    let _ = worker.join();
    print!("\r{} {}\n", paint("✓", Color::Green), label);
    let _ = io::stdout().flush();
}

fn setup(show_existing: bool) -> io::Result<Paths> {
    let p = paths();
    let first = !p.config_file.exists();

    if first {
        spinner("Preparing your local Aegis workspace...", Duration::from_millis(650));
    }
    fs::create_dir_all(&p.config_dir)?;
    fs::create_dir_all(&p.data_dir)?;
    fs::create_dir_all(&p.cache_dir)?;
    fs::create_dir_all(&p.model_dir)?;

    if first {
        let model = p
            .model_dir
            .join("aegis-model.gguf")
            .to_string_lossy()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        fs::write(
            &p.config_file,
            DEFAULT_CONFIG.replace("models/aegis-model.gguf", &model),
        )?;
        println!("\n{} First-run setup complete.", paint("✦", Color::Cyan));
        println!("  {} {}", blue("Config"), p.config_file.display());
        println!("  {} {}", blue("Models"), p.model_dir.display());
        println!("  {} {}", blue("Data  "), p.data_dir.display());
        println!("  {} {}", blue("Cache "), p.cache_dir.display());
        println!("\n{}", muted("No model was downloaded. Model downloads always require your consent."));
    } else if show_existing {
        println!("{} Aegis is already initialized.", paint("✓", Color::Green));
        println!("  Config: {}", p.config_file.display());
        println!("  Models: {}", p.model_dir.display());
    }
    Ok(p)
}

fn check_config(path: &Path) -> io::Result<()> {
    let config = aegis_core::AegisConfig::load(path)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    println!(
        "{} Agent: {} | backend: {} | model: {}",
        paint("✓", Color::Green),
        config.agent.name,
        config.model.backend,
        config.model.model_path
    );
    Ok(())
}

fn profile() {
    let p = aegis_hardware::HardwareProfile::detect();
    println!("{} {}", blue("◈"), "Aegis hardware profile");
    println!("  OS / architecture: {} / {}", p.operating_system, p.architecture);
    println!("  Logical CPU cores: {}", p.logical_cores);
    print_memory("Total RAM", p.total_memory_mb);
    print_memory("Available RAM", p.available_memory_mb);
    println!("  Resource tier: {}", p.resource_tier.as_str());
    println!("  Suggested context: {} tokens", p.recommended_context_tokens);
    print_memory("Model memory budget", p.model_memory_budget_mb);
}

fn print_memory(label: &str, value: Option<u64>) {
    match value {
        Some(value) => println!("  {label}: {value} MiB"),
        None => println!("  {label}: unavailable"),
    }
}

fn banner() {
    println!();
    println!("{}", blue("   ▄▀█ █▀▀ █▀▀ █ ▒ ▒"));
    println!("{}", blue("   █▀█ █▄▄ █▄▄ █ ░ ░"));
    println!("   {}", paint("A E G I S", Color::Rgb { r: 120, g: 190, b: 255 }).bold());
    println!("   {}", muted("LOCAL INTELLIGENCE  •  TERMINAL EDITION"));
    println!();
}

fn interactive(paths: &Paths) -> io::Result<()> {
    banner();
    println!(
        " {} {}",
        paint("●", Color::Yellow),
        muted("Local model runtime is not connected in this build")
    );
    println!(" {} {}", paint("⌂", Color::Blue), muted(env::current_dir()?.display().to_string().as_str()));
    println!(" {}", muted("Type /help for commands. Ctrl+D or /exit to leave."));
    println!();

    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    loop {
        print!("{} ", paint("›", Color::Rgb { r: 90, g: 170, b: 255 }).bold());
        io::stdout().flush()?;
        line.clear();
        if input.read_line(&mut line)? == 0 {
            println!();
            break;
        }
        let message = line.trim();
        if message.is_empty() {
            continue;
        }
        match message {
            "/exit" | "/quit" => break,
            "/help" => {
                println!("{} {}", blue("Commands"), muted("────────────────────────"));
                println!("  {}  Show this help", blue("/help"));
                println!("  {}  Inspect hardware resources", blue("/profile"));
                println!("  {}  Show local model configuration", blue("/model"));
                println!("  {}  Show setup directories", blue("/setup"));
                println!("  {}  Exit Aegis", blue("/exit"));
            }
            "/profile" => profile(),
            "/setup" => {
                println!("{} {}", blue("Config"), paths.config_file.display());
                println!("{} {}", blue("Models"), paths.model_dir.display());
                println!("{} {}", blue("Data"), paths.data_dir.display());
                println!("{} {}", blue("Cache"), paths.cache_dir.display());
            }
            "/model" => match aegis_core::AegisConfig::load(&paths.config_file) {
                Ok(config) => {
                    let model = PathBuf::from(&config.model.model_path);
                    println!("Backend: {}", config.model.backend);
                    println!("Model: {}", model.display());
                    println!(
                        "Model file: {}",
                        if model.is_file() { "found" } else { "not installed" }
                    );
                    println!("{}", muted("Inference engine: not connected yet"));
                }
                Err(error) => println!("{} {error}", paint("✖", Color::Red)),
            },
            _ => {
                println!(
                    "{} {}",
                    paint("◇", Color::Cyan),
                    "Aegis cannot generate a response yet: the local inference engine is not connected."
                );
                println!(
                    "{}",
                    muted("Your prompt was not sent to a server or written to a file.")
                );
            }
        }
        println!();
    }
    println!("{} Session ended.", muted("◈"));
    Ok(())
}
