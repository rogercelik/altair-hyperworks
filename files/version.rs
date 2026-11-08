//! Static build info for Altair Hyperworks.

pub const BUILD_VERSION: &str  = "v2.0.16";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "91dc10c73cd8";
pub const BUILD_TIMESTAMP: u64 = 1763220279;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Altair Hyperworks", BUILD_VERSION, BUILD_CHANNEL)
}
