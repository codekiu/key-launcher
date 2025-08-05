use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
};

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
        // Function keys
        "f1" => Some(Code::F1),
        "f2" => Some(Code::F2),
        "f3" => Some(Code::F3),
        "f4" => Some(Code::F4),
        "f5" => Some(Code::F5),
        "f6" => Some(Code::F6),
        "f7" => Some(Code::F7),
        "f8" => Some(Code::F8),
        "f9" => Some(Code::F9),
        "f10" => Some(Code::F10),
        "f11" => Some(Code::F11),
        "f12" => Some(Code::F12),
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

    // Create event loop (required for macOS)
    let event_loop = EventLoop::new().unwrap();

    // Create global hotkey manager on the main thread
    let manager = match GlobalHotKeyManager::new() {
        Ok(manager) => {
            println!("✅ Global hotkey manager created successfully");
            manager
        }
        Err(e) => {
            eprintln!("❌ Failed to create global hotkey manager: {}", e);
            eprintln!("💡 You may need to grant accessibility permissions to this application");
            std::process::exit(1);
        }
    };

    println!("🟢 {} started successfully!", APP_NAME);
    println!("📋 Leader key: {} ({:?})", config.leader_key, modifier);
    println!("🔗 Available bindings:");

    let mut hotkey_map = HashMap::new();

    // Register hotkeys with detailed debugging
    for (key_str, binding) in &config.bindings {
        println!("🔧 Processing binding: {} -> {}", key_str, binding.name);

        if let Some(code) = string_to_code(key_str) {
            let hotkey = HotKey::new(Some(modifier), code);
            println!(
                "   Created hotkey: {:?} + {:?} (ID: {})",
                modifier,
                code,
                hotkey.id()
            );

            match manager.register(hotkey) {
                Ok(()) => {
                    println!(
                        "   ✅ {} + {} → {} (ID: {})",
                        config.leader_key,
                        key_str,
                        binding.name,
                        hotkey.id()
                    );
                    hotkey_map.insert(hotkey.id(), binding);
                }
                Err(e) => {
                    eprintln!(
                        "   ❌ Failed to register hotkey {} + {}: {}",
                        config.leader_key, key_str, e
                    );
                }
            }
        } else {
            eprintln!("   ❌ Invalid key '{}' in config", key_str);
        }
    }

    if hotkey_map.is_empty() {
        eprintln!("❌ No valid hotkeys were registered");
        std::process::exit(1);
    }

    println!("\n📊 Registered {} hotkeys:", hotkey_map.len());
    for (id, binding) in &hotkey_map {
        println!("   ID {} -> {}", id, binding.name);
    }

    // Create a hidden window for the event loop
    let window = WindowBuilder::new()
        .with_title("Key Launcher")
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    println!("\n💡 Press Ctrl+C to stop or close this window");
    println!("✨ Hotkeys are now active and will be intercepted (no key bleed-through)");

    // Get global hotkey event receiver
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    // Run the event loop
    event_loop
        .run(move |event, window_target| {
            // Check for global hotkey events
            if let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
                println!("🔍 Received hotkey event with ID: {}", hotkey_event.id);
                if let Some(binding) = hotkey_map.get(&hotkey_event.id) {
                    execute_command(binding);
                } else {
                    println!("⚠️  No binding found for hotkey ID: {}", hotkey_event.id);
                    println!(
                        "   Available IDs: {:?}",
                        hotkey_map.keys().collect::<Vec<_>>()
                    );
                }
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => window_target.exit(),
                _ => (),
            }
        })
        .unwrap();
}
