use anyhow::{Context, Result};
use crate::utils::config_dir;
use crate::core_engine::CoreEngine;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const PID_FILE: &str = "hazeveil.pid";

pub async fn start(level: u8, pattern: Option<String>, exclude_app: Option<String>) -> Result<()> {
    let pid = std::process::id();
    let pid_path = config_dir()?.join(PID_FILE);
    std::fs::write(&pid_path, pid.to_string())?;
    let excluded: Vec<String> = exclude_app.map(|a| vec![a]).unwrap_or_default();
    let pat = pattern.unwrap_or_else(|| "random".to_string());
    println!("HazeVeil started (PID {})", pid);
    println!("Level: {} | Pattern: {} | Excluded: {:?}", level, pat, excluded);
    println!("Running... Ctrl+C to stop.");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || { r.store(false, Ordering::SeqCst); })
        .context("Error setting Ctrl+C handler")?;
    let mut engine = CoreEngine::new(level, &pat, running.clone());
    engine.run().await;
    let _ = std::fs::remove_file(&pid_path);
    println!("\nHazeVeil stopped.");
    Ok(())
}

pub fn stop() -> Result<()> {
    let pid_path = config_dir()?.join(PID_FILE);
    if !pid_path.exists() {
        println!("HazeVeil is not running.");
        return Ok(());
    }
    let pid_str = std::fs::read_to_string(&pid_path)?;
    let pid: i32 = pid_str.trim().parse().context("Invalid PID")?;
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;
    kill(Pid::from_raw(pid), Signal::SIGTERM).context("Failed to send SIGTERM")?;
    println!("Sent SIGTERM to PID {}.", pid);
    let _ = std::fs::remove_file(&pid_path);
    Ok(())
}

pub fn status() -> Result<()> {
    let pid_path = config_dir()?.join(PID_FILE);
    if pid_path.exists() {
        let pid = std::fs::read_to_string(&pid_path)?;
        println!("HazeVeil is running (PID {}).", pid.trim());
    } else {
        println!("HazeVeil is not running.");
    }
    Ok(())
}