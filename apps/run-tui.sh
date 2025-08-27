#!/bin/bash
# Run the TUI app from the project root
cd "$(dirname "$0")/.."
cargo run --release -p holosim-tui