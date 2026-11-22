# Aegis architecture

## Initial direction

Aegis is an agent runtime, not a foundation-model training project.

1. **Provider layer:** a stable interface for hosted APIs and local OpenAI-compatible inference servers.
2. **Agent core:** conversation state, planning, bounded tool loops, and result evaluation.
3. **Tool registry:** capability discovery; load only the relevant tool descriptions when needed.
4. **Execution backends:** local Rust tools first, with optional Cloudflare Code Mode for JavaScript orchestration.
5. **Permissions:** explicit allowlists, timeouts, resource limits, and approval gates for destructive or external side effects.
6. **Memory:** project-scoped persistent state kept outside model context until relevant.

## Code Mode boundary

Cloudflare Code Mode is an optional code-orchestration mechanism. It does not itself provide access to Ghidra, a local shell, Git, or arbitrary native executables. Aegis must implement adapters and a deliberate bridge to any local or remote execution service. Never expose an unrestricted local shell directly to generated cloud code.

## First implementation milestones

- [x] Rust workspace and CLI scaffold
- [x] TOML configuration and validation
- [ ] Provider trait and OpenAI-compatible client
- [ ] Streaming responses and cancellation
- [ ] Tool schema registry and discovery
- [ ] Sandboxed execution interface
- [ ] Git and filesystem adapters
- [ ] Ghidra bridge
- [ ] Optional Code Mode backend
- [ ] Persistent project memory
