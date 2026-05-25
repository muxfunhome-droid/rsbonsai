#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Configuration
REPO_URL="https://github.com/muxfunhome-droid/rsbonsai.git"
BINARY_NAME="rsbonsai"
INSTALL_DIR="$HOME/.rsbonsai"

echo "🌿 Starting rsbonsai installation..."

# 1. Check for basic dependencies
for cmd in git curl; do
    if ! command -v $cmd &> /dev/null; then
        echo "❌ Error: $cmd is not installed. Please install it first."
        exit 1
    fi
done

# 2. Check for Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo "🦀 Rust not found. Installing Rust toolchain via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Update current shell path to include cargo bin
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully."
else
    echo "✅ Rust is already installed."
fi

# 3. Clone the repository
if [ -d "$INSTALL_DIR" ]; then
    echo "📂 Directory $INSTALL_DIR already exists. Updating..."
    cd "$INSTALL_DIR" && git pull
else
    echo "📥 Cloning repository..."
    git clone "$REPO_URL" "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

# 4. Compile the project
echo "🔨 Compiling rsbonsai in release mode (this may take a few minutes)..."
cargo build --release

# 5. Setup the binary
# We move the binary to ~/.cargo/bin so it's available in the user's PATH
echo "🚀 Finalizing installation..."
cp "target/release/$BINARY_NAME" "$HOME/.cargo/bin/$BINARY_NAME"

echo "----------------------------------------------------------------"
echo "🎉 Installation Complete!"
echo "----------------------------------------------------------------"
echo "You can now run the generator by typing:"
echo "  $BINARY_NAME"
echo ""
echo "Example commands:"
echo "  $BINARY_NAME --live"
echo "  $BINARY_NAME --message \"My Zen Garden\" --life 30"
echo "----------------------------------------------------------------"
