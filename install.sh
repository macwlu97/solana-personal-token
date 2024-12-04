#!/bin/bash

# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Rust (cargo) is not installed. Please install Rust: https://www.rust-lang.org/learn/get-started"
    exit 1
fi

# Installing anchor-cli
echo "Installing anchor-cli..."
cargo install --git https://github.com/coral-xyz/anchor --tag v0.27.0 anchor-cli --locked

# Confirmation of installation completion
echo "anchor-cli installation completed."
