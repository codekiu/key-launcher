# Key Launcher Makefile

.PHONY: help build run install clean dev check format test start stop status daemon logs

# Default target
help:
	@echo "🚀 Key Launcher - Available commands:"
	@echo ""
	@echo "Development:"
	@echo "  make build    - Build the project in release mode"
	@echo "  make dev      - Build and run in development mode"
	@echo "  make run      - Build and run in release mode"
	@echo "  make check    - Run clippy and format checks"
	@echo "  make format   - Format code with rustfmt"
	@echo "  make test     - Run tests"
	@echo ""
	@echo "Installation:"
	@echo "  make install  - Install to /usr/local/bin (requires sudo)"
	@echo "  make clean    - Clean build artifacts"
	@echo ""
	@echo "Background Service:"
	@echo "  make daemon   - Run as background daemon"
	@echo "  make start    - Start background service"
	@echo "  make stop     - Stop background service"
	@echo "  make status   - Check service status"
	@echo "  make logs     - View daemon logs"
	@echo ""

# Build targets
build:
	@echo "🔨 Building Key Launcher in release mode..."
	cargo build --release

dev:
	@echo "🔨 Building and running in development mode..."
	cargo run

run: build
	@echo "🚀 Running Key Launcher..."
	./target/release/key-launcher

# Code quality
check:
	@echo "🔍 Running clippy..."
	cargo clippy -- -D warnings
	@echo "✅ Checking format..."
	cargo fmt -- --check

format:
	@echo "📝 Formatting code..."
	cargo fmt

test:
	@echo "🧪 Running tests..."
	cargo test

# Installation
install: build
	@echo "📦 Installing Key Launcher to /usr/local/bin..."
	sudo cp target/release/key-launcher /usr/local/bin/
	@echo "✅ Installation complete! You can now run 'key-launcher' from anywhere"

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Background service management
daemon: build
	@echo "🌙 Starting Key Launcher as background daemon..."
	@if pgrep -f "key-launcher" > /dev/null 2>&1; then \
		echo "⚠️  Key Launcher is already running. Use 'make stop' first."; \
	else \
		nohup ./target/release/key-launcher > key-launcher.log 2>&1 & \
		echo "✅ Key Launcher started in background (PID: $$!)"; \
		echo "📋 Use 'make logs' to view output"; \
	fi

start: daemon

stop:
	@echo "🛑 Stopping Key Launcher..."
	@if pgrep -f "key-launcher" > /dev/null 2>&1; then \
		pkill -f "key-launcher"; \
		echo "✅ Key Launcher stopped"; \
	else \
		echo "ℹ️  Key Launcher is not running"; \
	fi

status:
	@echo "📊 Key Launcher Status:"
	@if pgrep -f "key-launcher" > /dev/null 2>&1; then \
		echo "✅ Running (PID: $$(pgrep -f 'key-launcher'))"; \
		ps aux | grep key-launcher | grep -v grep; \
	else \
		echo "❌ Not running"; \
	fi

logs:
	@if [ -f key-launcher.log ]; then \
		echo "📋 Key Launcher logs:"; \
		tail -f key-launcher.log; \
	else \
		echo "❌ No log file found. Is the daemon running?"; \
	fi

# Platform-specific targets
install-macos: build
	@echo "🍎 Installing Key Launcher for macOS..."
	sudo cp target/release/key-launcher /usr/local/bin/
	@echo "💡 You may need to grant accessibility permissions in System Preferences"

install-linux: build
	@echo "🐧 Installing Key Launcher for Linux..."
	sudo cp target/release/key-launcher /usr/local/bin/
	@echo "💡 You may need to add your user to the 'input' group: sudo usermod -a -G input $$USER"
