//! Lightweight, dependency-free hardware profiling for adaptive local inference.
//!
//! Detection is intentionally conservative: unknown platform memory is left as
//! unknown rather than guessed. Linux and Android use /proc/meminfo.

use std::{env, fs, thread};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceTier {
    Tiny,
    Constrained,
    Standard,
    HighMemory,
    Unknown,
}

impl ResourceTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Constrained => "constrained",
            Self::Standard => "standard",
            Self::HighMemory => "high-memory",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareProfile {
    pub architecture: &'static str,
    pub operating_system: &'static str,
    pub logical_cores: usize,
    pub total_memory_mb: Option<u64>,
    pub available_memory_mb: Option<u64>,
    pub resource_tier: ResourceTier,
    pub recommended_context_tokens: u32,
    /// Initial conservative budget for model weights, not a guarantee that a
    /// particular model will load or perform well.
    pub model_memory_budget_mb: Option<u64>,
}

impl HardwareProfile {
    pub fn detect() -> Self {
        let (total_memory_mb, available_memory_mb) = detect_memory_mb();
        let resource_tier = classify(available_memory_mb);
        let recommended_context_tokens = match resource_tier {
            ResourceTier::Tiny => 2_048,
            ResourceTier::Constrained => 4_096,
            ResourceTier::Standard => 8_192,
            ResourceTier::HighMemory => 16_384,
            ResourceTier::Unknown => 4_096,
        };
        let model_memory_budget_mb = available_memory_mb.map(|available| available * 55 / 100);

        Self {
            architecture: env::consts::ARCH,
            operating_system: env::consts::OS,
            logical_cores: thread::available_parallelism()
                .map(usize::from)
                .unwrap_or(1),
            total_memory_mb,
            available_memory_mb,
            resource_tier,
            recommended_context_tokens,
            model_memory_budget_mb,
        }
    }
}

fn classify(available_mb: Option<u64>) -> ResourceTier {
    match available_mb {
        Some(mb) if mb < 1_024 => ResourceTier::Tiny,
        Some(mb) if mb < 3_072 => ResourceTier::Constrained,
        Some(mb) if mb < 8_192 => ResourceTier::Standard,
        Some(_) => ResourceTier::HighMemory,
        None => ResourceTier::Unknown,
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn detect_memory_mb() -> (Option<u64>, Option<u64>) {
    let Ok(contents) = fs::read_to_string("/proc/meminfo") else {
        return (None, None);
    };

    let mut total = None;
    let mut available = None;
    for line in contents.lines() {
        let mut fields = line.split_whitespace();
        let Some(key) = fields.next() else { continue };
        let Some(value) = fields.next().and_then(|v| v.parse::<u64>().ok()) else {
            continue;
        };
        // /proc/meminfo reports these values in KiB.
        match key {
            "MemTotal:" => total = Some(value / 1024),
            "MemAvailable:" => available = Some(value / 1024),
            _ => {}
        }
    }
    (total, available)
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn detect_memory_mb() -> (Option<u64>, Option<u64>) {
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_tiers_are_conservative() {
        assert_eq!(classify(Some(512)), ResourceTier::Tiny);
        assert_eq!(classify(Some(2_048)), ResourceTier::Constrained);
        assert_eq!(classify(Some(6_000)), ResourceTier::Standard);
        assert_eq!(classify(Some(16_000)), ResourceTier::HighMemory);
        assert_eq!(classify(None), ResourceTier::Unknown);
    }

    #[test]
    fn profile_has_at_least_one_worker() {
        assert!(HardwareProfile::detect().logical_cores >= 1);
    }
}
