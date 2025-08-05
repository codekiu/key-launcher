#!/bin/bash

# Key Launcher Installation Script

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Key Launcher Installation Script${NC}"
echo ""

# Detect OS
OS=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo -e "${RED}❌ Unsupported operating system: $OSTYPE${NC}"
    exit 1
fi

echo -e "${BLUE}📱 Detected OS: $OS${NC}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo is not installed${NC}"
    echo -e "${YELLOW}💡 Install Rust from: https://rustup.rs/${NC}"
    exit 1
fi

# Install platform-specific dependencies
if [[ "$OS" == "linux" ]]; then
    echo -e "${BLUE}🐧 Installing Linux dependencies...${NC}"
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y libx11-dev libxtst-dev libevdev-dev
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y libX11-devel libXtst-devel libevdev-devel
    elif command -v pacman &> /dev/null; then
        sudo pacman -S libx11 libxtst libevdev
    else
        echo -e "${YELLOW}⚠️  Could not detect package manager. Please install X11 development libraries manually.${NC}"
    fi
fi

# Build the project
echo -e "${BLUE}🔨 Building Key Launcher...${NC}"
cargo build --release

# Install binary
echo -e "${BLUE}📦 Installing binary...${NC}"
sudo cp target/release/key-launcher /usr/local/bin/
sudo chmod +x /usr/local/bin/key-launcher

echo -e "${GREEN}✅ Binary installed to /usr/local/bin/key-launcher${NC}"

# Install as system service
read -p "Do you want to install Key Launcher as a system service? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [[ "$OS" == "linux" ]]; then
        echo -e "${BLUE}🐧 Installing systemd service...${NC}"
        sudo cp key-launcher.service /etc/systemd/system/key-launcher@.service
        
        echo -e "${YELLOW}💡 To enable for your user, run:${NC}"
        echo "  systemctl --user enable key-launcher@$USER.service"
        echo "  systemctl --user start key-launcher@$USER.service"
        
    elif [[ "$OS" == "macos" ]]; then
        echo -e "${BLUE}🍎 Installing macOS LaunchAgent...${NC}"
        mkdir -p ~/Library/LaunchAgents
        cp com.keylauncher.daemon.plist ~/Library/LaunchAgents/
        
        echo -e "${YELLOW}💡 To enable the service, run:${NC}"
        echo "  launchctl load ~/Library/LaunchAgents/com.keylauncher.daemon.plist"
        echo "  launchctl start com.keylauncher.daemon"
    fi
fi

# Platform-specific setup instructions
if [[ "$OS" == "macos" ]]; then
    echo ""
    echo -e "${YELLOW}🍎 macOS Setup Instructions:${NC}"
    echo "1. Go to System Preferences → Security & Privacy → Privacy"
    echo "2. Select 'Accessibility' from the left panel"
    echo "3. Click the lock to make changes"
    echo "4. Add Terminal (or your terminal app) to the list"
    echo "5. You may also need to add 'key-launcher' if running as a service"
    
elif [[ "$OS" == "linux" ]]; then
    echo ""
    echo -e "${YELLOW}🐧 Linux Setup Instructions:${NC}"
    echo "If you encounter permission issues, try:"
    echo "  sudo usermod -a -G input $USER"
    echo "Then log out and back in."
fi

echo ""
echo -e "${GREEN}🎉 Installation complete!${NC}"
echo ""
echo -e "${BLUE}📋 Usage:${NC}"
echo "  key-launcher          # Run in foreground"
echo "  make daemon           # Run in background"
echo "  make stop             # Stop background service"
echo "  make status           # Check status"
echo ""
echo -e "${BLUE}⚙️  Configuration:${NC}"
echo "  Edit config.toml to customize key bindings"
echo ""
echo -e "${YELLOW}💡 First run will create a default config.toml file${NC}"
