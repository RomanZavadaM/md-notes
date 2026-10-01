#!/bin/sh
# MD Notes START: builds and runs the app from source (macOS / Linux).
# Requires Rust (https://rustup.rs), Node.js 20+ and the Tauri prerequisites:
# https://v2.tauri.app/start/prerequisites/
set -e
command -v cargo >/dev/null 2>&1 || { echo "Rust/cargo not found. Install it from https://rustup.rs"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "Node.js/npm not found. Install it from https://nodejs.org"; exit 1; }
cd "$(dirname "$0")/app"
npm ci
npm run tauri dev
