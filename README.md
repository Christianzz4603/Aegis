# Aegis

**Autonomous Engineering & General Intelligence System**

Aegis is a modular, Rust-first AI engineering agent. It uses existing AI models through provider adapters and orchestrates development and analysis tools through a permissioned runtime. It does not train a foundation model from scratch.

## Current status

Early project scaffold. This commit establishes the Rust workspace, configuration model, and model provenance. Agent planning, inference, tool adapters, and execution backends are not implemented yet.

## Initial model

Aegis is configured to target:

- **Model:** [lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled](https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled)
- **Base:** Qwen/Qwen3.6-35B-A3B
- **Format:** Transformers / Safetensors
- **Upstream license:** Apache-2.0 (verify the model card and license before redistribution)
- **Training provenance:** The model card describes supervised fine-tuning using the public reasoning-distillation dataset listed below.

The checkpoint is approximately **45.7 GB**. Model weights are deliberately **not** stored in this GitHub repository. GitHub source control is for Aegis code and configuration; obtain model weights directly from Hugging Face when setting up a suitable inference host. The checkpoint is too large for ordinary Git commits and would make the source repository cumbersome.

Related upstream resources:

- [Model checkpoint and model card](https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled)
- [Base model](https://huggingface.co/Qwen/Qwen3.6-35B-A3B)
- [SFT dataset](https://huggingface.co/datasets/lordx64/reasoning-distill-opus-4-7-max-sft)
- [Source reasoning dataset](https://huggingface.co/datasets/lordx64/reasoning-distill-claude-opus-4-7-max)

This is a Qwen model fine-tuned on reasoning examples attributed by its author to Claude Opus 4.7. It is **not Claude Opus**, and its performance should be evaluated independently.

## Design principles

- Rust-first agent core.
- Existing models through replaceable provider adapters.
- Local-first execution, with optional cloud backends.
- No mandatory MCP dependency.
- Cloudflare Code Mode can be an optional orchestration backend, not a hard dependency.
- Progressive tool discovery and compact tool results.
- Explicit permissions, bounded execution, and approval for destructive actions.
- Model weights stay outside the source repository.

## Workspace

```text
crates/
  aegis-core/   Shared configuration and core types
  aegis-cli/    Initial command-line application
config/
  default.toml  Default model and runtime configuration
docs/
  architecture.md
  model.md
```

## Build

Requires a recent stable Rust toolchain.

```bash
cargo build --workspace
cargo test --workspace
cargo run -p aegis-cli -- about
cargo run -p aegis-cli -- check-config --config config/default.toml
```

The initial CLI validates configuration only. It does not download model weights, start an inference server, or execute system tools.

## License

Aegis's project license has not yet been selected. The upstream model's license applies to that model, not automatically to Aegis source code.
