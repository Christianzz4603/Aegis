# First-run and terminal behavior

Running Aegis without a subcommand bootstraps user-local directories and opens the interactive terminal. Setup is idempotent: an existing config is preserved and never overwritten.

The bootstrap creates a config directory, a data directory with a models subdirectory, and a cache directory. It follows XDG locations on Linux/macOS and standard application-data locations on Windows.

First run does not download model weights, install packages, change system settings, or request elevated privileges. Model selection and download are separate explicit actions because model files can consume many gigabytes.

The terminal is the primary conversation surface. When local inference is integrated, ordinary prompts should be answered directly in the terminal. Files should only be created when the user requests them or approves a concrete output action.

Current limitation: the interactive terminal and setup are present, but no inference backend is wired into the chat loop yet. The CLI states this clearly rather than fabricating an answer.
