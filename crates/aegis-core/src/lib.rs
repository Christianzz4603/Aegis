use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct AegisConfig {
    pub agent: AgentConfig,
    pub model: ModelConfig,
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub max_tool_rounds: u32,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub model_id: String,
    pub base_url: String,
    pub api_key_env: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub mode: String,
    pub require_approval_for_destructive_actions: bool,
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
        if self.model.model_id.trim().is_empty() {
            return Err(ConfigError::Invalid("model.model_id cannot be empty".into()));
        }
        if !(self.model.base_url.starts_with("http://") || self.model.base_url.starts_with("https://")) {
            return Err(ConfigError::Invalid("model.base_url must use http:// or https://".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let config = AegisConfig {
            agent: AgentConfig { name: "Aegis".into(), max_tool_rounds: 12 },
            model: ModelConfig {
                provider: "openai-compatible".into(),
                model_id: "lordx64/Qwen3.6-35B-A3B-Claude-4.7-Opus-Reasoning-Distilled".into(),
                base_url: "http://127.0.0.1:8000/v1".into(),
                api_key_env: None,
            },
            runtime: RuntimeConfig {
                mode: "local".into(),
                require_approval_for_destructive_actions: true,
            },
        };
        assert!(config.validate().is_ok());
    }
}
