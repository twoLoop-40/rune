// Session persistence — save/restore conversation to disk.

use std::path::{Path, PathBuf};

use claude_code_core::message::SerializedMessage;

/// Session storage location.
fn sessions_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".rune").join("sessions")
}

/// Generate a session file path.
fn session_path(session_id: &str) -> PathBuf {
    sessions_dir().join(format!("{session_id}.json"))
}

/// Save conversation to disk.
pub fn save_session(session_id: &str, messages: &[SerializedMessage]) -> anyhow::Result<()> {
    let dir = sessions_dir();
    std::fs::create_dir_all(&dir)?;
    let path = session_path(session_id);
    let json = serde_json::to_string_pretty(messages)?;
    std::fs::write(&path, json)?;
    Ok(())
}

/// Load conversation from disk.
pub fn load_session(session_id: &str) -> anyhow::Result<Vec<SerializedMessage>> {
    let path = session_path(session_id);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let json = std::fs::read_to_string(&path)?;
    let messages: Vec<SerializedMessage> = serde_json::from_str(&json)?;
    Ok(messages)
}

/// List recent sessions.
pub fn list_sessions() -> anyhow::Result<Vec<(String, std::time::SystemTime)>> {
    let dir = sessions_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let modified = entry.metadata()?.modified()?;
            sessions.push((name, modified));
        }
    }
    sessions.sort_by(|a, b| b.1.cmp(&a.1)); // newest first
    Ok(sessions)
}

/// Generate a new session ID based on timestamp.
pub fn new_session_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("s_{}", now.as_secs())
}
