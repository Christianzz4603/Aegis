# Initial model

## Selected checkpoint

- Repository: https://huggingface.co/lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled
- Base model: Qwen/Qwen3.6-35B-A3B
- Format: Transformers / Safetensors
- Model-card license: Apache-2.0
- Approximate repository size: 45.7 GB

The model card describes supervised fine-tuning with Unsloth and TRL on the author's reasoning-distillation dataset. It attributes the teacher traces to Claude Opus 4.7. This is a fine-tuned Qwen checkpoint, not an Anthropic model.

## Weight handling

Do not commit model weights to Aegis. They are large binary artifacts and should be obtained directly from Hugging Face by the person operating the inference host. Keep local model files under an ignored `models/` directory or in the inference runtime's own cache.

## Runtime compatibility

The upstream model card documents Transformers and vLLM usage. Aegis initially targets an OpenAI-compatible HTTP endpoint so the inference engine can be changed without changing the agent core.

The configured endpoint is a placeholder:
`http://127.0.0.1:8000/v1`

No model server is started by the initial Aegis CLI.
