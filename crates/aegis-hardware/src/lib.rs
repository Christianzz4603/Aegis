//! Lightweight, dependency-free hardware profiling for adaptive local inference.
//!
//! Memory figures are conservative estimates. On Linux/Android, Aegis combines
//! host memory information with cgroup limits when those limits are readable.
//! A model must still be checked against the selected runtime's measured needs.

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
    /// Initial budget estimate for model weights, not a model-fit guarantee.
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

        // Keep 512 MiB for the OS, the agent, and other applications before
        // assigning a conservative fraction of the remainder to model weights.
        let model_memory_budget_mb =
            available_memory_mb.map(|available| available.saturating_sub(512) * 55 / 100);

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
    let host = fs::read_to_string("/proc/meminfo")
        .ok()
        .map(|contents| parse_meminfo(&contents))
        .unwrap_or((None, None));

    let cgroup = detect_cgroup_memory_mb();
    combine_memory(host, cgroup)
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn detect_memory_mb() -> (Option<u64>, Option<u64>) {
    (None, None)
}

fn parse_meminfo(contents: &str) -> (Option<u64>, Option<u64>) {
    let mut total = None;
    let mut available = None;

    for line in contents.lines() {
        let mut fields = line.split_whitespace();
        let Some(key) = fields.next() else {
            continue;
        };
        let Some(value) = fields.next().and_then(|value| value.parse::<u64>().ok()) else {
            continue;
        };

        // Linux /proc/meminfo reports these values in KiB.
        match key {
            "MemTotal:" => total = Some(value / 1_024),
            "MemAvailable:" => available = Some(value / 1_024),
            _ => {}
        }
    }

    (total, available)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn detect_cgroup_memory_mb() -> (Option<u64>, Option<u64>) {
    // cgroup v2
    let v2_limit = read_cgroup_value("/sys/fs/cgroup/memory.max")
        .as_deref().and_then(parse_cgroup_limit_mb);
    let v2_current = read_cgroup_value("/sys/fs/cgroup/memory.current")
        .as_deref().and_then(parse_cgroup_usage_mb);

    if v2_limit.is_some() {
        return (v2_limit, v2_limit.zip(v2_current).map(|(limit, used)| {
            limit.saturating_sub(used)
        }));
    }

    // cgroup v1, including the common Android memory-controller mount.
    let limit = [
        "/sys/fs/cgroup/memory/memory.limit_in_bytes",
        "/dev/memcg/memory.limit_in_bytes",
    ]
    .into_iter()
    .find_map(|path| read_cgroup_value(path).as_deref().and_then(parse_cgroup_limit_mb));

    let current = [
        "/sys/fs/cgroup/memory/memory.usage_in_bytes",
        "/dev/memcg/memory.usage_in_bytes",
    ]
    .into_iter()
    .find_map(|path| read_cgroup_value(path).as_deref().and_then(parse_cgroup_usage_mb));

    (limit, limit.zip(current).map(|(limit, used)| limit.saturating_sub(used)))
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn detect_cgroup_memory_mb() -> (Option<u64>, Option<u64>) {
    (None, None)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn read_cgroup_value(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|value| value.trim().to_owned())
}

fn parse_cgroup_limit_mb(value: &str) -> Option<u64> {
    if value == "max" {
        return None;
    }

    let bytes = value.parse::<u64>().ok()?;
    // cgroup v1 commonly uses a very large number to mean "unlimited".
    if bytes >= (1_u64 << 60) {
        return None;
    }
    parse_bytes_mb(bytes)
}

fn parse_bytes_mb(bytes: u64) -> Option<u64> {
    Some(bytes / 1_048_576)
}

fn parse_cgroup_usage_mb(value: &str) -> Option<u64> {
    value.parse::<u64>().ok().and_then(parse_bytes_mb)
}

fn combine_memory(
    host: (Option<u64>, Option<u64>),
    cgroup: (Option<u64>, Option<u64>),
) -> (Option<u64>, Option<u64>) {
    let total = match (host.0, cgroup.0) {
        (Some(host), Some(limit)) => Some(host.min(limit)),
        (Some(host), None) => Some(host),
        (None, Some(limit)) => Some(limit),
        (None, None) => None,
    };

    let available = match (host.1, cgroup.1.or(cgroup.0)) {
        (Some(host), Some(limit_or_remaining)) => Some(host.min(limit_or_remaining)),
        (Some(host), None) => Some(host),
        (None, Some(limit_or_remaining)) => Some(limit_or_remaining),
        (None, None) => None,
    };

    (total, available)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_tiers_are_conservative() {
        assert_eq!(classify(Some(512)), ResourceTier::Tiny);
        assert_eq!(classify(Some(2_048)), ResourceTier::Constrained);
        assert_eq!(classify(Some(6_000)), ResourceTier::Standard);
        assert_eq!(classify(Some(16_000)), ResourceTier::HighMemory);
        assert_eq!(classify(None), ResourceTier::Unknown);
    }

    #[test]
    fn parses_linux_meminfo_in_kibibytes() {
        let input = "MemTotal:       4194304 kB\nMemAvailable:   2097152 kB\n";
        assert_eq!(parse_meminfo(input), (Some(4_096), Some(2_048)));
    }

    #[test]
    fn parses_cgroup_limits_and_unlimited_values() {
        assert_eq!(parse_cgroup_limit_mb("1073741824"), Some(1_024));
        assert_eq!(parse_cgroup_limit_mb("max"), None);
        assert_eq!(parse_cgroup_limit_mb("9223372036854771712"), None);
    }

    #[test]
    fn parses_cgroup_usage_values_in_bytes() {
        assert_eq!(parse_cgroup_usage_mb("536870912"), Some(512));
        assert_eq!(parse_cgroup_usage_mb("not-a-number"), None);
    }

    #[test]
    fn cgroup_limit_caps_host_memory() {
        assert_eq!(
            combine_memory((Some(8_192), Some(6_000)), (Some(2_048), Some(1_024))),
            (Some(2_048), Some(1_024))
        );
    }

    #[test]
    fn profile_has_at_least_one_worker() {
        assert!(HardwareProfile::detect().logical_cores >= 1);
    }

    #[test]
    fn model_budget_reserves_headroom() {
        let profile = HardwareProfile::detect();
        if let Some(available) = profile.available_memory_mb {
            assert_eq!(
                profile.model_memory_budget_mb,
                Some(available.saturating_sub(512) * 55 / 100)
            );
        }
    }
}
