# Aegis Adaptive Engine

Aegis should select a conservative local inference configuration based on the
machine that is actually running it, not merely its advertised hardware.

## First implementation slice

The `aegis-hardware` crate currently detects:

- Operating system and CPU architecture from Rust compile-time target data.
- Available logical CPU parallelism.
- Total and available memory on Linux and Android through `/proc/meminfo`.
- A conservative resource tier and initial model-memory budget.

The profiler has no third-party dependencies. Unknown memory is represented as
`None`; it is never fabricated.

## Resource tiers

| Tier | Available memory | Initial context suggestion |
|---|---:|---:|
| Tiny | Under 1 GiB | 2,048 tokens |
| Constrained | 1 to under 3 GiB | 4,096 tokens |
| Standard | 3 to under 8 GiB | 8,192 tokens |
| High memory | 8 GiB or more | 16,384 tokens |
| Unknown | Not detectable | 4,096 tokens |

These are bootstrap heuristics, not model compatibility guarantees. The initial
model memory budget is 55% of currently available memory. A later model catalog
and runtime benchmark must account for weight format, KV cache, compute buffers,
OS overhead, thermal limits, and actual inference performance.

## Next milestones

1. Add platform-specific memory probes for Windows, macOS, and BSD.
2. Add a model catalog with measured RAM, context, and architecture requirements.
3. Benchmark BitNet and llama.cpp backends on each supported target.
4. Make runtime configuration adapt dynamically under memory pressure.
5. Add a safe interactive agent loop and explicit tool permissions.

Aegis remains local-only. Hardware profiling and runtime selection must not send
machine details, prompts, source code, or model data to a remote service.
