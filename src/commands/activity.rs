use crate::prefix::PrefixResolver;
use crate::store::{Store, StoreError};
use chrono::{DateTime, Utc};
use colored::*;
use std::path::Path;

/// A parsed log entry
struct LogEntry {
    timestamp: DateTime<Utc>,
    author: String,
    message: String,
    task_title: String,
    short_id: String,
}

pub fn activity(path: &Path, limit: usize, all: bool) -> Result<(), StoreError> {
    let store = Store::open(path)?;

    let tasks = if all {
        store.list_all()?
    } else {
        store.list_active()?
    };

    // Resolve shortest unique prefixes across ALL tasks (including closed/cancelled)
    // This ensures displayed prefixes work with `bt edit`, which searches all directories
    let resolver = PrefixResolver::new(&store)?;

    let mut entries: Vec<LogEntry> = Vec::new();

    for (_, task) in &tasks {
        let short_id = resolver.shortest_prefix(task.id()).to_string();

        // Parse log entries from the task's log field
        // Format: ### 2025-11-26T23:41:41Z Author Name\n\nMessage content\n
        for section in task.log.split("\n### ") {
            let section = section.trim();
            if section.is_empty() {
                continue;
            }

            // Parse the header line: "2025-11-26T23:41:41Z Author Name"
            let lines: Vec<&str> = section.lines().collect();
            if lines.is_empty() {
                continue;
            }

            let header = lines[0].trim_start_matches("### ");
            let parts: Vec<&str> = header.splitn(2, ' ').collect();
            if parts.len() < 2 {
                continue;
            }

            // Try to parse timestamp
            let timestamp_str = parts[0];
            let timestamp = match DateTime::parse_from_rfc3339(timestamp_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => continue,
            };

            let author = parts[1].to_string();

            // Get the message (skip empty lines after header)
            let message: String = lines[1..]
                .iter()
                .skip_while(|l| l.is_empty())
                .take(1)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("");

            entries.push(LogEntry {
                timestamp,
                author,
                message,
                task_title: task.title().to_string(),
                short_id: short_id.clone(),
            });
        }
    }

    // Sort by timestamp descending (most recent first)
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Limit results
    let entries: Vec<_> = entries.into_iter().take(limit).collect();

    if entries.is_empty() {
        println!("{}", "No recent activity.".dimmed());
        return Ok(());
    }

    for entry in entries {
        let time_str = entry.timestamp.format("%Y-%m-%d %H:%M").to_string();
        println!(
            "{}  {}  {}",
            time_str.dimmed(),
            entry.short_id.cyan(),
            entry.task_title
        );
        println!(
            "    {} {}",
            entry.author.dimmed(),
            entry.message
        );
    }

    Ok(())
}
