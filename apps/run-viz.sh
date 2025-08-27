#!/bin/bash
# Run the visualization app from the project root
cd "$(dirname "$0")/.."
cargo run --release -p holosim-viz