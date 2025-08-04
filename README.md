# 🚀 Key Launcher

A fast, configurable system-wide key launcher written in Rust. Launch any application or execute any command with customizable key combinations.

## ✨ Features

- **🎯 System-wide hotkeys** - Works regardless of which application has focus
- **⚙️ Fully configurable** - Customize leader key and all key bindings via TOML config
- **🔥 Fast & lightweight** - Built in Rust for maximum performance
- **🌍 Cross-platform** - Works on macOS, Linux, and Windows
- **📝 Auto-generated config** - Creates a default configuration file on first run
- **🛡️ Safe execution** - Proper error handling and validation

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or higher
- On Linux: X11 development libraries (`sudo apt-get install libx11-dev libxtst-dev libevdev-dev` on Ubuntu/Debian)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/key-launcher.git
cd key-launcher
```

2. Build and run:
```bash
cargo run
```

3. On first run, a `config.toml` file will be created with default bindings.

## ⚙️ Configuration

The application uses a `config.toml` file for all configuration:

```toml
leader_key = "alt"

[bindings.b]
name = "Brave Browser"
command = "open"
args = ["-a", "Brave Browser"]

[bindings.t]
name = "Terminal"
command = "open"
args = ["-a", "Alacritty"]

[bindings.v]
name = "VS Code"
command = "code"
args = []
```

### Leader Keys

Supported leader keys:
- `alt` - Alt key
- `ctrl` or `control` - Control key  
- `cmd`, `meta`, or `super` - Command/Super/Windows key
- `shift` - Shift key
- `space` - Space bar
- `tab` - Tab key
- `escape` or `esc` - Escape key

### Key Bindings

Each binding consists of:
- **Key**: Any letter (a-z), number (0-9), or `space`
- **Name**: Display name for the application/command
- **Command**: The executable to run
- **Args**: Array of command-line arguments

### Platform-Specific Examples

#### macOS
```toml
[bindings.b]
name = "Brave Browser"
command = "open"
args = ["-a", "Brave Browser"]
```

#### Linux
```toml
[bindings.f]
name = "Firefox"
command = "firefox"
args = ["--new-window"]
```

#### Windows
```toml
[bindings.n]
name = "Notepad"
command = "notepad.exe"
args = []
```

## 🎮 Usage

1. **Start the application**: `cargo run`
2. **Press your leader key** (default: Alt) + any configured key
3. **Release to execute** the bound command

Example with default config:
- `Alt + B` → Opens Brave Browser
- `Alt + T` → Opens Terminal (Alacritty)
- `Alt + D` → Opens DB Browser for SQLite

## 🛠️ Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/key-launcher.git
cd key-launcher

# Build in debug mode
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Project Structure

```
key-launcher/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Rust dependencies and metadata
├── config.toml          # Configuration file (auto-generated)
├── README.md            # This file
└── LICENSE              # License information
```

## 🔧 Dependencies

- **[rdev](https://crates.io/crates/rdev)** - Cross-platform input event handling
- **[serde](https://crates.io/crates/serde)** - Serialization/deserialization
- **[toml](https://crates.io/crates/toml)** - TOML configuration parsing

## 🐛 Troubleshooting

### Permission Issues (macOS)
On macOS, you may need to grant accessibility permissions:
1. Go to System Preferences → Security & Privacy → Privacy
2. Select "Accessibility" from the left panel
3. Add your terminal application or the compiled binary

### Permission Issues (Linux)
On some Linux distributions, you may need to run with sudo or add your user to the `input` group:
```bash
sudo usermod -a -G input $USER
```
Then log out and back in.

### Key Not Working
- Check that the key string in your config matches the supported format
- Ensure no other application is capturing the same key combination
- Try a different leader key if there are conflicts

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Add tests** if applicable
5. **Commit your changes**: `git commit -m 'Add amazing feature'`
6. **Push to the branch**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Development Guidelines

- Follow Rust naming conventions
- Add documentation for public functions
- Include tests for new features
- Update README.md for significant changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [rdev](https://github.com/Narsil/rdev) for cross-platform input handling
- The Rust community for excellent tooling and libraries

## ⭐ Show Your Support

If you find this project useful, please consider giving it a star on GitHub! It helps others discover the project and motivates continued development.

---

**Made with ❤️ and Rust**
