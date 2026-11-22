# Aegis

**Autonomous Engineering & General Intelligence System**

Aegis is a local-first, terminal-based AI coding and engineering agent, written primarily in Rust. Its goal is a Codex/Claude Code-style workflow with an open-weight language model running on the same machine as the CLI.

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

Useful commands: `aegis`, `aegis setup`, `aegis profile`, `aegis check-config`, and `aegis about`. Inside the terminal: `/help`, `/profile`, `/model`, `/setup`, and `/exit`.

## Build from source

```bash
cargo build --release --package aegis-cli
cargo test --workspace
./target/release/aegis
```

## First-run locations

Aegis respects XDG paths on Linux/macOS and standard application-data paths on Windows. Config is stored under the user config directory; models and app data under the user data directory; cache under the user cache directory.

## CI binaries

GitHub Actions checks formatting, runs workspace tests, builds the workspace, then compiles release CLI binaries for Linux x86_64, Windows x86_64, and macOS ARM64. Each binary is uploaded as a workflow artifact. CI runs for pull requests, pushes to `main`, version tags, and manual dispatches.

## Current implementation status

The Rust workspace, hardware profiling, config validation, idempotent first-run bootstrap, and interactive terminal shell are implemented. **Local model inference and the AI agent/tool loop are not connected yet.** Until then, Aegis reports that it has no model connected rather than pretending to answer prompts. No model weights are bundled or silently downloaded.

## License

Aegis's source-code license has not yet been selected. Check each upstream model's license separately.
