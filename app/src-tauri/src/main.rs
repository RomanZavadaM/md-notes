// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    md_notes_lib::run()
}

// Audit-only CI trigger for stable v1.0.0 verification. Do not merge.
