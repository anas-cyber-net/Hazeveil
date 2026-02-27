use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod core_engine;
mod mouse_engine;
mod keyboard_engine;
mod touchpad_engine;
mod scroll_engine;
mod timing_engine;
mod window_engine;
mod clipboard_engine;
mod audio_engine;
mod ai_pattern;
mod context_detector;
mod utils;
mod daemon;

#[derive(Parser)]
#[command(
    name = "hazeveil",
    about = "HazeVeil — Linux Behavioral Veil",
    version = "0.1.0",
    args_conflicts_with_subcommands = false,
)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Start {
        #[arg(long, default_value_t = 3)]
        level: u8,
        #[arg(long)]
        pattern: Option<String>,
        #[arg(long)]
        exclude_app: Option<String>,
    },
    Stop,
    TrainPattern {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "10m")]
        duration: String,
    },
    Status,
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(log_level)
    ).init();
    print_banner();
    match cli.command {
        None => { run_setup().await?; }
        Some(Commands::Setup) => { run_setup().await?; }
        Some(Commands::Start { level, pattern, exclude_app }) => {
            if level < 1 || level > 5 {
                eprintln!("Error: --level must be between 1 and 5");
                std::process::exit(1);
            }
            daemon::start(level, pattern, exclude_app).await?;
        }
        Some(Commands::Stop) => { daemon::stop()?; }
        Some(Commands::TrainPattern { name, duration }) => {
            ai_pattern::train_pattern(&name, &duration).await?;
        }
        Some(Commands::Status) => { daemon::status()?; }
        Some(Commands::Config) => { run_setup().await?; }
    }
    Ok(())
}

async fn run_setup() -> Result<()> {
    println!("=== HazeVeil Setup ===");
    println!();
    println!("Choose simulation level:");
    println!("  [1] Minimal  — very subtle, nearly undetectable");
    println!("  [2] Low      — light variation");
    println!("  [3] Moderate — balanced (recommended)");
    println!("  [4] High     — strong variation");
    println!("  [5] Maximum  — maximum behavioral masking");
    println!();
    print!("Enter level [1-5] (default: 3): ");
    use std::io::{self, Write, BufRead};
    io::stdout().flush()?;
    let stdin = io::stdin();
    let line = stdin.lock().lines().next()
        .unwrap_or(Ok(String::new())).unwrap_or_default();
    let line = line.trim().to_string();
    let level: u8 = if line.is_empty() { 3 }
        else { line.parse::<u8>().unwrap_or(3).clamp(1, 5) };
    println!();
    println!("Choose pattern:");
    println!("  [1] random     — blend different patterns each session (recommended)");
    println!("  [2] casual     — casual browser style");
    println!("  [3] developer  — developer/keyboard heavy");
    println!("  [4] gamer      — fast and precise");
    println!("  [5] methodical — slow and careful");
    println!();
    print!("Enter choice [1-5] (default: 1): ");
    io::stdout().flush()?;
    let line2 = stdin.lock().lines().next()
        .unwrap_or(Ok(String::new())).unwrap_or_default();
    let line2 = line2.trim().to_string();
    let pattern = match line2.as_str() {
        "2" => "casual-browser",
        "3" => "developer",
        "4" => "gamer",
        "5" => "methodical",
        _   => "random",
    }.to_string();
    println!();
    println!("✓ Level: {} | Pattern: {}", level, pattern);
    println!();
    println!("Starting HazeVeil...");
    println!();
    daemon::start(level, Some(pattern), None).await?;
    Ok(())
}

fn print_banner() {
    println!(r#"
   $$\   $$\                                                          $$\ $$\
   $$ |  $$ |                                                         \__|$$ |
   $$ |  $$ | $$$$$$\  $$$$$$$$\  $$$$$$\        $$\    $$\  $$$$$$\  $$\ $$ |
   $$$$$$$$ | \____$$\ \____$$  |$$  __$$\       \$$\  $$  |$$  __$$\ $$ |$$ |
   $$  __$$ | $$$$$$$ |  $$$$ _/ $$$$$$$$ |       \$$\$$  / $$$$$$$$ |$$ |$$ |
   $$ |  $$ |$$  __$$ | $$  _/   $$   ____|        \$$$  /  $$   ____|$$ |$$ |
   $$ |  $$ |\$$$$$$$ |$$$$$$$$\ \$$$$$$$\          \$  /   \$$$$$$$\ $$ |$$ |
   \__|  \__| \_______|\________| \_______|          \_/     \_______|\__|\__|

   Behavioral Variation Simulator v0.1.0 | Ethical Research Only
"#);
}