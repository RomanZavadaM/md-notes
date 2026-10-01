@echo off
rem MD Notes START: builds and runs the app from source (Windows).
rem Requires Rust (https://rustup.rs), Node.js 20+ and the Tauri prerequisites:
rem https://v2.tauri.app/start/prerequisites/
setlocal
where cargo >nul 2>nul || (echo Rust/cargo not found. Install it from https://rustup.rs & pause & exit /b 1)
where npm >nul 2>nul || (echo Node.js/npm not found. Install it from https://nodejs.org & pause & exit /b 1)
cd /d "%~dp0app" || exit /b 1
call npm ci || (pause & exit /b 1)
call npm run tauri dev
endlocal
