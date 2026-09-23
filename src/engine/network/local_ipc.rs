use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use super::lan_discovery::DiscoveredServer;

fn get_ipc_file() -> std::path::PathBuf {
    std::env::temp_dir().join("minecraft_lan_hosts.txt")
}

pub fn register_local_host(motd: &str, port: u16) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let line = format!("{}|{}|127.0.0.1:{}\n", now, motd, port);
    let path = get_ipc_file();
    let mut existing = fs::read_to_string(&path).unwrap_or_default();
    if !existing.is_empty() && !existing.ends_with('\n') {
        existing.push('\n');
    }
    existing.push_str(&line);
    let _ = fs::write(&path, existing);
}

pub fn poll_local_hosts() -> Vec<DiscoveredServer> {
    let path = get_ipc_file();
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let mut servers = Vec::new();
    let mut valid_lines = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            if let Ok(ts) = parts[0].parse::<u64>() {
                if now.saturating_sub(ts) < 5 {
                    valid_lines.push(line);
                    let motd = parts[1].to_string();
                    let addr = parts[2].to_string();
                    if !servers.iter().any(|s: &DiscoveredServer| s.address == addr) {
                        servers.push(DiscoveredServer::new(motd, addr));
                    }
                }
            }
        }
    }
    let mut out = valid_lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    let _ = fs::write(&path, out);
    servers
}
