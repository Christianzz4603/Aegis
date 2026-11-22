# Aegis

**Autonomous Engineering & General Intelligence System**

Aegis is a local-first, terminal-based AI coding and engineering agent written primarily in Rust. The CLI uses a blue terminal theme, animated startup indicators, compact status icons, and an interactive prompt inspired by modern coding agents.

## Project direction

- **Local inference only:** no hosted AI providers, remote inference APIs, or API keys.
- **Terminal-native:** an interactive CLI that works in the current project directory.
- **Rust-first:** agent loop, terminal UX, project context, permissions, and tool orchestration belong in Rust.
- **Local tools:** filesystem, search, editing, shell, browser, and platform adapters are planned.
- **No mandatory MCP:** use native Rust tool adapters and compact capability discovery.

## Install and run

Download the standalone binary for your operating system from a successful GitHub Actions workflow run's artifacts. Running the binary does not require Rust.

```bash
./aegis
```

On first launch, Aegis creates user-local config, models, data, and cache directories, writes a config file, and opens the interactive terminal. It does not download model weights or install packages without consent.

Commands: `aegis`, `aegis setup`, `aegis profile`, `aegis check-config`, and `aegis about`. Inside the terminal: `/help`, `/profile`, `/model`, `/setup`, and `/exit`.

Set `NO_COLOR=1` to disable ANSI colors.

## Build from source

```bash
cargo build --release --package aegis-cli
cargo test --workspace
./target/release/aegis
```

## Current implementation status

The Rust workspace, hardware profiling, config validation, first-run bootstrap, and themed interactive terminal are implemented. **Local model inference and the AI agent/tool loop are not connected yet.** Until then, Aegis reports that it has no model connected rather than pretending to answer prompts. No model weights are bundled or silently downloaded.

## CI binaries

GitHub Actions checks formatting, runs workspace tests, builds the workspace, then compiles standalone release CLI binaries for supported targets and uploads them as workflow artifacts.
