use std::time::Instant;

#[derive(Clone, Debug)]
pub struct DiscoveredServer {
    pub motd: String,
    pub address: String,
    pub last_seen: Instant,
}

impl DiscoveredServer {
    pub fn new(motd: String, address: String) -> Self {
        Self { motd, address, last_seen: Instant::now() }
    }
}

pub fn create_ping_string(motd: &str, address: &str) -> String {
    format!("[MOTD]{}[/MOTD][AD]{}[/AD]", motd, address)
}

pub fn parse_motd(ping: &str) -> String {
    let start_tag = "[MOTD]";
    let end_tag = "[/MOTD]";
    if let Some(start) = ping.find(start_tag) {
        let content_start = start + start_tag.len();
        if let Some(end) = ping[content_start..].find(end_tag) {
            return ping[content_start..content_start + end].to_string();
        }
    }
    "missing no".to_string()
}

pub fn parse_address(ping: &str) -> Option<String> {
    let end_motd = "[/MOTD]";
    let end_motd_idx = ping.find(end_motd)?;
    let remainder = &ping[end_motd_idx + end_motd.len()..];
    if remainder.contains(end_motd) {
        return None;
    }
    let start_ad = "[AD]";
    let end_ad = "[/AD]";
    let ad_idx = remainder.find(start_ad)?;
    let content_start = ad_idx + start_ad.len();
    let ad_end_idx = remainder[content_start..].find(end_ad)?;
    Some(remainder[content_start..content_start + ad_end_idx].to_string())
}
