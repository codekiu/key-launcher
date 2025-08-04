use rdev::{listen, Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

const CONFIG_FILE: &str = "config.toml";
const APP_NAME: &str = "Key Launcher";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    leader_key: String,
    bindings: HashMap<String, AppBinding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppBinding {
    name: String,
    command: String,
    args: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut bindings = HashMap::new();

        bindings.insert(
            "b".to_string(),
            AppBinding {
                name: "Brave Browser".to_string(),
                command: "open".to_string(),
                args: vec!["-a".to_string(), "Brave Browser".to_string()],
            },
        );

        bindings.insert(
            "t".to_string(),
            AppBinding {
                name: "Alacritty".to_string(),
                command: "open".to_string(),
                args: vec!["-a".to_string(), "Alacritty".to_string()],
            },
        );

        bindings.insert(
            "d".to_string(),
            AppBinding {
                name: "DB Browser for SQLite".to_string(),
                command: "open".to_string(),
                args: vec!["-a".to_string(), "DB Browser for SQLite".to_string()],
            },
        );

        Config {
            leader_key: "alt".to_string(),
            bindings,
        }
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    if !std::path::Path::new(CONFIG_FILE).exists() {
        println!("📄 Config file not found, creating default {}", CONFIG_FILE);
        let default_config = Config::default();
        let toml_string = toml::to_string_pretty(&default_config)?;
        fs::write(CONFIG_FILE, toml_string)?;
        println!("✅ Created default configuration file");
        return Ok(default_config);
    }

    let config_content = fs::read_to_string(CONFIG_FILE)?;
    let config: Config = toml::from_str(&config_content)
        .map_err(|e| format!("Failed to parse {}: {}", CONFIG_FILE, e))?;

    // Validate config
    if config.bindings.is_empty() {
        return Err("Configuration file has no key bindings defined".into());
    }

    Ok(config)
}

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "alt" => Some(Key::Alt),
        "ctrl" | "control" => Some(Key::ControlLeft),
        "cmd" | "meta" | "super" => Some(Key::MetaLeft),
        "shift" => Some(Key::ShiftLeft),
        "space" => Some(Key::Space),
        "tab" => Some(Key::Tab),
        "escape" | "esc" => Some(Key::Escape),
        // Single character keys
        "a" => Some(Key::KeyA),
        "b" => Some(Key::KeyB),
        "c" => Some(Key::KeyC),
        "d" => Some(Key::KeyD),
        "e" => Some(Key::KeyE),
        "f" => Some(Key::KeyF),
        "g" => Some(Key::KeyG),
        "h" => Some(Key::KeyH),
        "i" => Some(Key::KeyI),
        "j" => Some(Key::KeyJ),
        "k" => Some(Key::KeyK),
        "l" => Some(Key::KeyL),
        "m" => Some(Key::KeyM),
        "n" => Some(Key::KeyN),
        "o" => Some(Key::KeyO),
        "p" => Some(Key::KeyP),
        "q" => Some(Key::KeyQ),
        "r" => Some(Key::KeyR),
        "s" => Some(Key::KeyS),
        "t" => Some(Key::KeyT),
        "u" => Some(Key::KeyU),
        "v" => Some(Key::KeyV),
        "w" => Some(Key::KeyW),
        "x" => Some(Key::KeyX),
        "y" => Some(Key::KeyY),
        "z" => Some(Key::KeyZ),
        // Numbers
        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),
        _ => None,
    }
}

fn execute_command(binding: &AppBinding) {
    println!("🚀 Launching: {}", binding.name);

    let mut cmd = Command::new(&binding.command);
    for arg in &binding.args {
        cmd.arg(arg);
    }

    match cmd.spawn() {
        Ok(_) => println!("✅ Successfully launched: {}", binding.name),
        Err(e) => println!("❌ Failed to launch {}: {}", binding.name, e),
    }
}

// Global state for the callback
static GLOBAL_STATE: OnceLock<GlobalState> = OnceLock::new();

#[derive(Debug)]
struct GlobalState {
    leader_key: Key,
    leader_pressed: AtomicBool,
    config: Config,
}

fn callback(event: Event) {
    let state = GLOBAL_STATE.get().unwrap();

    match event.event_type {
        EventType::KeyPress(key) => {
            if key == state.leader_key {
                state.leader_pressed.store(true, Ordering::SeqCst);
            } else if state.leader_pressed.load(Ordering::SeqCst) {
                // Convert the pressed key back to string to match config
                let key_string = match key {
                    Key::KeyA => "a",
                    Key::KeyB => "b",
                    Key::KeyC => "c",
                    Key::KeyD => "d",
                    Key::KeyE => "e",
                    Key::KeyF => "f",
                    Key::KeyG => "g",
                    Key::KeyH => "h",
                    Key::KeyI => "i",
                    Key::KeyJ => "j",
                    Key::KeyK => "k",
                    Key::KeyL => "l",
                    Key::KeyM => "m",
                    Key::KeyN => "n",
                    Key::KeyO => "o",
                    Key::KeyP => "p",
                    Key::KeyQ => "q",
                    Key::KeyR => "r",
                    Key::KeyS => "s",
                    Key::KeyT => "t",
                    Key::KeyU => "u",
                    Key::KeyV => "v",
                    Key::KeyW => "w",
                    Key::KeyX => "x",
                    Key::KeyY => "y",
                    Key::KeyZ => "z",
                    Key::Num0 => "0",
                    Key::Num1 => "1",
                    Key::Num2 => "2",
                    Key::Num3 => "3",
                    Key::Num4 => "4",
                    Key::Num5 => "5",
                    Key::Num6 => "6",
                    Key::Num7 => "7",
                    Key::Num8 => "8",
                    Key::Num9 => "9",
                    Key::Space => "space",
                    _ => return, // Ignore other keys
                };

                if let Some(binding) = state.config.bindings.get(key_string) {
                    execute_command(binding);
                }
            }
        }
        EventType::KeyRelease(key) => {
            if key == state.leader_key {
                state.leader_pressed.store(false, Ordering::SeqCst);
            }
        }
        _ => {}
    }
}

fn main() {
    println!("🚀 {} v{}", APP_NAME, VERSION);

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    let leader_key = match string_to_key(&config.leader_key) {
        Some(key) => key,
        None => {
            eprintln!("❌ Invalid leader key '{}'. Supported keys: alt, ctrl, cmd, shift, space, tab, escape, a-z, 0-9", config.leader_key);
            std::process::exit(1);
        }
    };

    println!("🟢 {} started successfully!", APP_NAME);
    println!("📋 Leader key: {}", config.leader_key);
    println!("🔗 Available bindings:");
    for (key, binding) in &config.bindings {
        println!("   {} + {} → {}", config.leader_key, key, binding.name);
    }
    println!("\n💡 Press Ctrl+C to stop");

    // Initialize global state
    let global_state = GlobalState {
        leader_key,
        leader_pressed: AtomicBool::new(false),
        config,
    };

    if let Err(_) = GLOBAL_STATE.set(global_state) {
        eprintln!("❌ Failed to initialize global state");
        std::process::exit(1);
    }

    if let Err(error) = listen(callback) {
        eprintln!("❌ Error listening to events: {:?}", error);
        eprintln!("💡 You may need to grant accessibility permissions to this application");
        std::process::exit(1);
    }
}
