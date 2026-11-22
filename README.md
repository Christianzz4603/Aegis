# Aegis

**Autonomous Engineering & General Intelligence System**

Aegis is a local-first, terminal-based AI coding and engineering agent, written primarily in Rust. Its goal is to provide a Codex/Claude Code-style workflow while running an open-weight language model on the same machine as the CLI.

## Non-negotiable project direction

- **Local inference only:** no hosted AI providers, remote inference APIs, or API keys.
- **Terminal-native:** Aegis is a CLI coding agent that works in the user's current project directory.
- **Existing model:** use an existing open-weight model; do not train a foundation model from scratch.
- **Rust-first:** agent loop, terminal UX, project context, permissions, and tool orchestration belong in Rust.
- **Local tools:** inspect files, search code, edit files, run builds and tests, and use installed developer tools with explicit permissions.
- **Optional Code Mode:** Cloudflare Code Mode may be explored for local code orchestration only if it can preserve the local-only privacy boundary. It is not required and must never send source code or prompts to a hosted model.
- **No mandatory MCP:** use native Rust tool adapters and compact capability discovery.

## Initial model

Aegis initially targets:

- **Checkpoint:** [lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled](https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled)
- **Base:** Qwen/Qwen3.6-35B-A3B
- **Upstream format:** Transformers / BF16 Safetensors
- **Upstream repository size:** approximately 45.7 GB
- **License shown on model card:** Apache-2.0 (check upstream terms before redistributing)
- **GGUF quantizations:** the model author documents community GGUF variants, including an IQ4_XS build around 18.9 GB.

This is a Qwen checkpoint fine-tuned on reasoning examples attributed by its author to Claude Opus 4.7. It is not Claude Opus or an Anthropic model.

### Model storage is local

The model is downloaded to the same computer that runs Aegis and is loaded by a local inference engine. The weights are not copied into this GitHub repository and are not sent to a remote inference provider.

The 45.7 GB figure is the upstream BF16 checkpoint's download/storage size, not a server requirement or an API charge. Quantized GGUF files are smaller, but the model still needs substantial memory bandwidth and RAM/VRAM to run well. Only about 3B parameters are active per token; that reduces compute per token, but does not mean the entire 35B model can fit in 3B worth of memory.

Aegis must detect available system resources and explain whether the selected model is practical before downloading or loading it. Smaller local models should also be supported.

Related resources:

- [Model checkpoint and model card](https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled)
- [Model GGUF quantization information](https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled-Q8_0-GGUF)
- [Base model](https://huggingface.co/Qwen/Qwen3.6-35B-A3B)

## Product behavior

Aegis should feel like a terminal coding agent:

1. Start in a repository or project directory.
2. Read project instructions and inspect relevant files.
3. Build a plan for the requested task.
4. Show proposed tool actions and request approval when appropriate.
5. Edit files and run local commands.
6. Inspect command output, tests, and diffs.
7. Iterate until the task is complete or explain what is blocked.

It must not silently perform destructive operations. Shell execution should be bounded, visible, cancellable, and permission-controlled.

## Architecture

```text
crates/
  aegis-cli/       Terminal application and interactive session
  aegis-core/      Agent loop, messages, task state
  aegis-model/     Local model runtime abstraction
  aegis-context/   Project discovery and context assembly
  aegis-tools/     Filesystem, search, patch, shell, git tools
  aegis-policy/    Approvals, allowlists, limits
  aegis-memory/    Local project/session memory
  aegis-tui/       Terminal UI
config/
  default.toml
docs/
  architecture.md
  model.md
```

## Current status

Early scaffold. The initial Rust workspace, CLI, TOML configuration, and model provenance are committed. Local model inference, interactive agent loop, editing tools, terminal UI, and approval workflow are not implemented yet.

## Build

Requires a recent stable Rust toolchain.

```bash
cargo build --workspace
cargo test --workspace
cargo run -p aegis-cli -- about
cargo run -p aegis-cli -- check-config --config config/default.toml
```

The current CLI validates configuration only. It does not yet download or run a model, edit files, or execute shell commands.

## License

Aegis's source-code license has not yet been selected. The upstream model's license applies to that model, not automatically to Aegis source code.
