use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct AegisConfig {
    pub agent: AgentConfig,
    pub model: ModelConfig,
    pub runtime: RuntimeConfig,
    #[serde(default)]
    pub adaptive: AdaptiveConfig,
}

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub max_tool_rounds: u32,
    pub approval_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    /// Local inference backend, for example "bitnet" or "llama.cpp".
    pub backend: String,
    pub format: String,
    pub model_path: String,
    pub context_length: u32,
    /// "auto" or a backend-specific layer count.
    pub gpu_layers: String,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub mode: String,
    pub working_directory: String,
    pub require_approval_for_destructive_actions: bool,
    pub allow_network: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AdaptiveConfig {
    pub enabled: bool,
    pub profile: String,
    pub reserve_memory_mb: u64,
    pub max_context_length: u32,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            profile: "auto".into(),
            reserve_memory_mb: 512,
            max_context_length: 16_384,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read config: {0}")]
    Read(#[from] std::io::Error),
    #[error("invalid TOML config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("invalid config: {0}")]
    Invalid(String),
}

impl AegisConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let source = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&source)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.agent.name.trim().is_empty() {
            return Err(ConfigError::Invalid("agent.name cannot be empty".into()));
        }
        if self.agent.max_tool_rounds == 0 {
            return Err(ConfigError::Invalid("agent.max_tool_rounds must be greater than zero".into()));
        }
        if self.model.backend.trim().is_empty() || self.model.model_path.trim().is_empty() {
            return Err(ConfigError::Invalid("model.backend and model.model_path are required".into()));
        }
        if self.model.context_length == 0 || self.adaptive.max_context_length == 0 {
            return Err(ConfigError::Invalid("context lengths must be greater than zero".into()));
        }
        if self.runtime.mode != "local" {
            return Err(ConfigError::Invalid("only local runtime mode is supported".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_parses_and_validates() {
        let source = include_str!("../../../config/default.toml");
        let config: AegisConfig = toml::from_str(source).expect("default TOML parses");
        config.validate().expect("default config is valid");
        assert!(config.adaptive.enabled);
        assert!(!config.runtime.allow_network);
    }
}
