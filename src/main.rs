use enigo::{Enigo, MouseControllable};
use serde::Deserialize;
use std::collections::VecDeque;
use std::fs;
use std::time::{Duration, Instant};
use std::thread::sleep;

const DEFAULT_WINDOW_DURATION: Duration = Duration::from_secs(60);
const DEFAULT_WAKEUP_DURATION: Duration = Duration::from_secs(10);
const DEFAULT_MOVEMENT_THRESHOLD: f64 = 0.05;

#[derive(Deserialize, Default)]
struct Config {
    movement_threshold: Option<f64>,
    timeout_seconds: Option<u64>,
    wakeup_period_seconds: Option<u64>,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
    time: Instant,
}

/// Locks the user system.
/// Any errors while trying to lock are logged but do not stop the application.
fn lock_system() {
    if let Err(e) = (|| -> Result<(), std::io::Error> {
        #[cfg(target_os = "windows")]
        std::process::Command::new("rundll32.exe").args(&["user32.dll,LockWorkStation"]).output()?;

        #[cfg(target_os = "linux")]
        std::process::Command::new("loginctl").arg("lock-session").output()?;

        #[cfg(target_os = "macos")]
        std::process::Command::new("/System/Library/CoreServices/Menu Extras/User.menu/Contents/Resources/CGSession").arg("-suspend").output()?;

        Ok(())
    })() {
        eprintln!("Failed to lock system: {}", e);
    }
}

/// Loads configuration from file. 
/// If there's any error reading the file or parsing it, defaults are returned.
fn load_config() -> Config {
    match fs::read_to_string("config.toml") {
        Ok(config_data) => match toml::from_str(&config_data) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to parse config file: {}", e);
                Config::default()
            },
        },
        Err(e) => {
            eprintln!("Failed to read config file: {}", e);
            Config::default()
        }
    }
}

fn main() {
    // Load configuration (or default values if there's an error).
    let config = load_config();
    let movement_threshold = config.movement_threshold.unwrap_or(DEFAULT_MOVEMENT_THRESHOLD);
    let window_duration = Duration::from_secs(config.timeout_seconds.unwrap_or(DEFAULT_WINDOW_DURATION.as_secs()));
    let wakeup_period = Duration::from_secs(config.wakeup_period_seconds.unwrap_or(DEFAULT_WAKEUP_DURATION.as_secs()));

    let capacity:usize = ((window_duration.as_secs() + wakeup_period.as_secs() - 1) / wakeup_period.as_secs()) as usize;  
    let mut positions: VecDeque<Position> = VecDeque::with_capacity(capacity);

    let enigo = Enigo::new();

    loop {
        let (x, y) = enigo.mouse_location();
        let current_position = Position { x, y, time: Instant::now() };
        positions.push_back(current_position.clone());
        
        if positions.len() > capacity {
            positions.pop_front();
        }

        if positions.len() == capacity {
            let old_position = &positions[0];
            let delta_x = (current_position.x - old_position.x).abs() as f64;
            let delta_y = (current_position.y - old_position.y).abs() as f64;

            let total_movement = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

            if total_movement < movement_threshold {
                lock_system();
            }
        }
        
        sleep(wakeup_period);
    }
}
