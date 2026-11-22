# Aegis architecture

## Product definition

Aegis is a terminal-native coding and engineering agent, similar in workflow to Codex CLI or Claude Code, but designed around local inference. The model runs on the same machine as Aegis. Prompts, source code, tool outputs, and model inference remain local by default.

## Core components

1. **Terminal UX:** interactive CLI session, streaming output, diffs, cancellation, and clear approval prompts.
2. **Local model runtime:** load local model files through an embedded/local inference backend. No hosted model providers, remote inference APIs, or API keys.
3. **Agent loop:** task planning, context selection, tool calls, observations, bounded retries, and final summaries.
4. **Project context:** detect repository root, read project instructions, inspect relevant files, and avoid flooding the context window.
5. **Native tool registry:** Rust adapters for filesystem reads, search, patch application, Git status/diff, and approved command execution.
6. **Policy layer:** working-directory boundaries, command allowlists, timeouts, cancellation, and approval gates for destructive or network actions.
7. **Local memory:** project/session notes stored on-device and retrieved only when relevant.

## Local inference boundary

The initial target is a local GGUF model through a llama.cpp-based runtime. Prefer an embedded library/bindings or a locally spawned inference process with no remote access. A loopback-only transport, if needed internally by the runtime, is not a hosted API and must not expose a network listener beyond localhost. The agent must not require an OpenAI-compatible remote endpoint.

The model runtime is replaceable so Aegis can support smaller GGUF models on modest hardware. Before downloading/loading a model, inspect available RAM, VRAM, storage, and model quantization size and provide a resource estimate.

## Tool execution

Tools execute on the same machine as the Aegis CLI. No Cloudflare account, MCP server, or cloud executor is required. Cloudflare Code Mode is only a possible future optional local orchestration experiment; it must not compromise the local-only guarantee.

## Safety and reliability

- Never run arbitrary generated shell commands without policy checks.
- Show commands before execution when approval is required.
- Keep operations inside the selected workspace unless explicitly approved.
- Use timeouts, output limits, cancellation, and structured errors.
- Require confirmation for destructive file operations, broad modifications, and network access.
- Show diffs before or after edits and preserve recoverability.

## Milestones

- [x] Initial Rust workspace and CLI scaffold
- [x] TOML configuration and validation
- [x] Local-only product direction and model provenance
- [ ] Terminal session and streaming UI
- [ ] Local model runtime abstraction
- [ ] GGUF inference backend
- [ ] Prompt and chat-template handling
- [ ] Agent loop and planning
- [ ] Project context and instruction files
- [ ] Filesystem/search/patch tools
- [ ] Approved shell and Git tools
- [ ] Diff review and test iteration
- [ ] Local project memory
