use crate::error::{RcgenError, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};
pub fn parse_date(date_str: &str) -> Result<DateTime<FixedOffset>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt);
    }
    let formats = [
        "%Y-%m-%d %H:%M:%S %z",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d",
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y",
        "%m/%d/%Y %H:%M:%S",
        "%m/%d/%Y",
    ];
    for format in &formats {
        if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
            return Ok(dt);
        }
        if let Ok(ndt) = NaiveDateTime::parse_from_str(date_str, format) {
            let dt: DateTime<FixedOffset> =
                DateTime::from_naive_utc_and_offset(ndt, FixedOffset::east_opt(0).unwrap());
            return Ok(dt);
        }
    }
    if let Some(dt) = parse_relative_date(date_str) {
        return Ok(dt);
    }
    Err(RcgenError::DateParse(format!(
        "Invalid date format: {}",
        date_str
    )))
}
fn parse_relative_date(date_str: &str) -> Option<DateTime<FixedOffset>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let re = Regex::new(r"(?i)(\d+)\s*(day|week|month|year|hour|minute|second)s?\s*(ago)?").ok()?;
    let caps = re.captures(date_str)?;
    let amount: i64 = caps.get(1)?.as_str().parse().ok()?;
    let unit = caps.get(2)?.as_str().to_lowercase();
    let is_ago = caps.get(3).is_some();
    let seconds = match unit.as_str() {
        "second" => amount,
        "minute" => amount * 60,
        "hour" => amount * 3600,
        "day" => amount * 86400,
        "week" => amount * 604800,
        "month" => amount * 2592000,
        "year" => amount * 31536000,
        _ => return None,
    };
    let timestamp = if is_ago { now - seconds } else { now + seconds };
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.with_timezone(&FixedOffset::east_opt(0).unwrap()))
}
pub fn format_date(date: &DateTime<FixedOffset>, format: &str) -> String {
    date.format(format).to_string()
}
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    let truncated = &s[..max_len - 3];
    format!("{}...", truncated)
}
pub fn get_file_extension(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}
pub fn is_binary_file(content: &[u8]) -> bool {
    content.iter().any(|&b| b == 0)
}
pub fn human_readable_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let base = 1024_f64;
    let bytes_f64 = bytes as f64;
    let exp = (bytes_f64.ln() / base.ln()).floor() as i32;
    let unit = UNITS[exp as usize];
    let size = bytes_f64 / base.powi(exp);
    format!("{:.1} {}", size, unit)
}
