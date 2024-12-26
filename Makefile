# ==========================================
# Stream-Sync Project Makefile for Windows
# ==========================================

# Commands
CARGO       = cargo
NPM         = npm
ECHO        = echo

# Project directories
RUST_DIR    = .
CLIENT_DIR  = client

# Default target
.DEFAULT_GOAL := help

# ==========================================
# Help Command
# ==========================================
.PHONY: help
help:
	@$(ECHO) Stream-Sync Project Commands:
	@$(ECHO)   make install    - Install all dependencies
	@$(ECHO)   make build      - Build both server and client
	@$(ECHO)   make run        - Run both server and client
	@$(ECHO)   make clean      - Clean all build artifacts
	@$(ECHO)   make help       - Show this help message
	@$(ECHO).
	@$(ECHO) Server Commands:
	@$(ECHO)   make server-build - Build Rust server
	@$(ECHO)   make server-run   - Run Rust server
	@$(ECHO).
	@$(ECHO) Client Commands:
	@$(ECHO)   make client-build - Build TypeScript client
	@$(ECHO)   make client-run   - Run TypeScript client

# ==========================================
# Installation Commands
# ==========================================
.PHONY: install
install: install-server install-client
	@$(ECHO) All dependencies installed successfully!

.PHONY: install-server
install-server:
	@$(ECHO) Installing Rust dependencies...
	cd $(RUST_DIR) && $(CARGO) check

.PHONY: install-client
install-client:
	@$(ECHO) Installing Node.js dependencies...
	cd $(CLIENT_DIR) && $(NPM) install

# ==========================================
# Build Commands
# ==========================================
.PHONY: build
build: server-build client-build
	@$(ECHO) Build completed successfully!

.PHONY: server-build
server-build:
	@$(ECHO) Building Rust server...
	cd $(RUST_DIR) && $(CARGO) build

.PHONY: client-build
client-build:
	@$(ECHO) Building TypeScript client...
	cd $(CLIENT_DIR) && $(NPM) run build

# ==========================================
# Run Commands
# ==========================================
.PHONY: run
run: run-server run-client

.PHONY: run-server
run-server:
	@$(ECHO) Starting Rust server...
	cd $(RUST_DIR) && $(CARGO) run

.PHONY: run-client
run-client:
	@$(ECHO) Starting TypeScript client...
	cd $(CLIENT_DIR) && $(NPM) start

# ==========================================
# Clean Commands
# ==========================================
.PHONY: clean
clean: clean-server clean-client
	@$(ECHO) Cleanup completed successfully!

.PHONY: clean-server
clean-server:
	@$(ECHO) Cleaning Rust server...
	cd $(RUST_DIR) && $(CARGO) clean

.PHONY: clean-client
clean-client:
	@$(ECHO) Cleaning TypeScript client...
	cd $(CLIENT_DIR) && if exist node_modules rmdir /s /q node_modules
	cd $(CLIENT_DIR) && $(NPM) run clean