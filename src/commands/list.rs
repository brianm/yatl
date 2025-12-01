use crate::prefix::PrefixResolver;
use crate::store::{Store, StoreError};
use crate::task::{Priority, Status};
use chrono::{DateTime, Utc};
use colored::*;
use serde::Serialize;
use std::path::Path;

/// JSON output structure for a task
#[derive(Serialize)]
struct TaskJson {
    id: String,
    short_id: String,
    title: String,
    status: Status,
    priority: Priority,
    tags: Vec<String>,
    blocked_by: Vec<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    author: Option<String>,
    body: String,
}

pub fn list(
    path: &Path,
    all: bool,
    long: bool,
    status_filter: Option<&str>,
    priority_filter: Option<&str>,
    tag_filter: Option<&str>,
    search_query: Option<&str>,
    limit: Option<usize>,
    json: bool,
    show_body: bool,
) -> Result<(), StoreError> {
    let store = Store::open(path)?;

    let tasks = if all {
        store.list_all()?
    } else {
        store.list_active()?
    };

    // Resolve shortest unique prefixes across ALL tasks (including closed/cancelled)
    // This ensures displayed prefixes work with `bt edit`, which searches all directories
    let resolver = PrefixResolver::new(&store)?;

    let mut json_tasks: Vec<TaskJson> = Vec::new();
    let mut count = 0;

    for (task_path, task) in &tasks {
        // Check limit
        if let Some(max) = limit {
            if count >= max {
                break;
            }
        }
        // Derive status from path
        let status = store.status_from_path(task_path).unwrap_or(Status::Open);

        // Apply filters
        if let Some(sf) = status_filter {
            if status.to_string() != sf {
                continue;
            }
        }

        if let Some(pf) = priority_filter {
            if task.priority().to_string() != pf {
                continue;
            }
        }

        // Filter by tag
        if let Some(tag) = tag_filter {
            if !task.frontmatter.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)) {
                continue;
            }
        }

        // Search in title and body
        if let Some(query) = search_query {
            let query_lower = query.to_lowercase();
            let title_matches = task.title().to_lowercase().contains(&query_lower);
            let body_matches = task.body.to_lowercase().contains(&query_lower);
            if !title_matches && !body_matches {
                continue;
            }
        }

        // Get shortest unique prefix for this task
        let short_id = resolver.shortest_prefix(task.id());

        if json {
            json_tasks.push(TaskJson {
                id: task.id().full().to_string(),
                short_id: short_id.to_string(),
                title: task.title().to_string(),
                status,
                priority: task.priority(),
                tags: task.frontmatter.tags.clone(),
                blocked_by: task.frontmatter.blocked_by.iter().map(|id| id.full().to_string()).collect(),
                created: task.frontmatter.created,
                updated: task.frontmatter.updated,
                author: task.frontmatter.author.clone(),
                body: task.body.clone(),
            });
        } else {
            let status_colored = match status {
                Status::Open => "open".green(),
                Status::InProgress => "in-progress".yellow(),
                Status::Blocked => "blocked".red(),
                Status::Closed => "closed".blue(),
                Status::Cancelled => "cancelled".red(),
            };

            let priority_colored = match task.priority() {
                Priority::Critical => "critical".red(),
                Priority::High => "high".yellow(),
                Priority::Medium => "medium".normal(),
                Priority::Low => "low".blue(),
            };

            if long {
                println!("{}", task.title().bold());
                println!("  ID: {}", short_id);
                println!(
                    "  Status: {}  Priority: {}",
                    status_colored, priority_colored
                );
                if show_body {
                    let preview = get_body_preview(&task.body, 200);
                    if !preview.is_empty() {
                        println!("  {}", preview.dimmed());
                    }
                }
                println!();
            } else {
                println!(
                    "{}\t{}\t{}\t{}",
                    short_id,
                    status_colored,
                    priority_colored,
                    task.title()
                );
                if show_body {
                    let preview = get_body_preview(&task.body, 80);
                    if !preview.is_empty() {
                        println!("    {}", preview.dimmed());
                    }
                }
            }
        }

        count += 1;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&json_tasks).unwrap_or_default());
    }

    Ok(())
}

/// Get a truncated preview of the body (first meaningful line, truncated to max_len)
fn get_body_preview(body: &str, max_len: usize) -> String {
    // Skip the log section and get the first non-empty line
    let preview = body
        .lines()
        .take_while(|line| !line.starts_with("## Log"))
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim();

    if preview.len() <= max_len {
        preview.to_string()
    } else {
        format!("{}...", &preview[..max_len - 3])
    }
}
