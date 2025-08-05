use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

fn string_to_modifier(key_str: &str) -> Option<Modifiers> {
    match key_str.to_lowercase().as_str() {
        "alt" => Some(Modifiers::ALT),
        "ctrl" | "control" => Some(Modifiers::CONTROL),
        "cmd" | "meta" | "super" => Some(Modifiers::SUPER),
        "shift" => Some(Modifiers::SHIFT),
        _ => None,
    }
}

fn string_to_code(key_str: &str) -> Option<Code> {
    match key_str.to_lowercase().as_str() {
        "space" => Some(Code::Space),
        "tab" => Some(Code::Tab),
        "escape" | "esc" => Some(Code::Escape),
        // Single character keys
        "a" => Some(Code::KeyA),
        "b" => Some(Code::KeyB),
        "c" => Some(Code::KeyC),
        "d" => Some(Code::KeyD),
        "e" => Some(Code::KeyE),
        "f" => Some(Code::KeyF),
        "g" => Some(Code::KeyG),
        "h" => Some(Code::KeyH),
        "i" => Some(Code::KeyI),
        "j" => Some(Code::KeyJ),
        "k" => Some(Code::KeyK),
        "l" => Some(Code::KeyL),
        "m" => Some(Code::KeyM),
        "n" => Some(Code::KeyN),
        "o" => Some(Code::KeyO),
        "p" => Some(Code::KeyP),
        "q" => Some(Code::KeyQ),
        "r" => Some(Code::KeyR),
        "s" => Some(Code::KeyS),
        "t" => Some(Code::KeyT),
        "u" => Some(Code::KeyU),
        "v" => Some(Code::KeyV),
        "w" => Some(Code::KeyW),
        "x" => Some(Code::KeyX),
        "y" => Some(Code::KeyY),
        "z" => Some(Code::KeyZ),
        // Numbers
        "0" => Some(Code::Digit0),
        "1" => Some(Code::Digit1),
        "2" => Some(Code::Digit2),
        "3" => Some(Code::Digit3),
        "4" => Some(Code::Digit4),
        "5" => Some(Code::Digit5),
        "6" => Some(Code::Digit6),
        "7" => Some(Code::Digit7),
        "8" => Some(Code::Digit8),
        "9" => Some(Code::Digit9),
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

fn main() {
    println!("🚀 {} v{}", APP_NAME, VERSION);

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    let modifier = match string_to_modifier(&config.leader_key) {
        Some(modifier) => modifier,
        None => {
            eprintln!(
                "❌ Invalid leader key '{}'. Supported keys: alt, ctrl, cmd, shift",
                config.leader_key
            );
            std::process::exit(1);
        }
    };

    println!("🟢 {} started successfully!", APP_NAME);
    println!("📋 Leader key: {}", config.leader_key);
    println!("🔗 Available bindings:");

    // Create global hotkey manager
    let manager = GlobalHotKeyManager::new().unwrap();
    let mut hotkey_map = HashMap::new();
    let mut hotkey_id = 1u32;

    // Register hotkeys
    for (key_str, binding) in &config.bindings {
        if let Some(code) = string_to_code(key_str) {
            let hotkey = HotKey::new(Some(modifier), code);

            match manager.register(hotkey) {
                Ok(()) => {
                    println!("   {} + {} → {}", config.leader_key, key_str, binding.name);
                    hotkey_map.insert(hotkey.id(), (key_str.clone(), binding.clone()));
                    hotkey_id += 1;
                }
                Err(e) => {
                    eprintln!(
                        "❌ Failed to register hotkey {} + {}: {}",
                        config.leader_key, key_str, e
                    );
                }
            }
        } else {
            eprintln!("❌ Invalid key '{}' in config", key_str);
        }
    }

    if hotkey_map.is_empty() {
        eprintln!("❌ No valid hotkeys were registered");
        std::process::exit(1);
    }

    println!("\n💡 Press Ctrl+C to stop");
    println!("✨ Hotkeys are now active and will be intercepted (no key bleed-through)");

    // Create event receiver
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    // Main event loop
    loop {
        if let Ok(event) = global_hotkey_channel.try_recv() {
            if let Some((key_str, binding)) = hotkey_map.get(&event.id) {
                execute_command(binding);
            }
        }

        // Small delay to prevent excessive CPU usage
        thread::sleep(Duration::from_millis(10));
    }
}
