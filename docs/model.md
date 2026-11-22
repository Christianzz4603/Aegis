# Initial local model

## Selected checkpoint

- Repository: https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled
- Base model: Qwen/Qwen3.6-35B-A3B
- Original format: BF16 Safetensors
- Upstream repository size: approximately 45.7 GB
- Model-card license: Apache-2.0
- Model family: 35B total parameters, approximately 3B active per token

The model card describes supervised fine-tuning using Unsloth and TRL on reasoning examples attributed by the author to Claude Opus 4.7. This is a fine-tuned Qwen model, not an Anthropic model.

## Local GGUF option

The model author documents GGUF quantizations. An IQ4_XS quantization is listed at approximately 18.9 GB; Q5_K_M is around 25 GB and Q8_0 around 35 GB. Actual download sizes and compatibility should be checked in the quantization repository before selecting a file.

GGUF quantization reduces disk and memory requirements compared with BF16, but an approximately 19 GB model is still not suitable for every computer. The fact that only around 3B parameters are active per token reduces compute, not the amount of model data that must be available to the inference engine. KV cache and runtime overhead also require memory.

## Local-only runtime

Aegis must load the model on the user's own machine using a local inference backend. It must not send prompts or project files to a hosted inference provider. No API key or cloud model account should be required.

The first implementation should investigate llama.cpp support for this model architecture and chat template before choosing a binding or vendoring a runtime. Do not claim compatibility until a real smoke test succeeds.

## Hardware awareness

Before model download, Aegis should show:
- model file size and available disk space;
- system RAM and available memory;
- GPU/VRAM availability where detectable;
- expected quantization and context-cache overhead;
- a clear warning if the selected model is unlikely to run acceptably.

Aegis should support smaller local models as alternatives. Do not assume the 35B model is the only supported model.

## Weight handling

Do not commit model weights to the Aegis Git repository. Download them directly to the user's local model directory, such as `models/`, which is ignored by Git.
