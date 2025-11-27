use crate::store::{Store, StoreError};
use crate::task::Priority;
use colored::*;
use std::io::{self, IsTerminal, Read};
use std::path::Path;

/// Read body from stdin if it's not a TTY (i.e., piped input)
fn read_stdin_body() -> Option<String> {
    if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() && !buffer.is_empty() {
            return Some(buffer.trim().to_string());
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
pub fn update(
    path: &Path,
    id: &str,
    title: Option<&str>,
    priority: Option<Priority>,
    tags: Option<Vec<String>>,
    add_tag: Option<&str>,
    remove_tag: Option<&str>,
    body: Option<&str>,
) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;
    let mut task = store.load(&task_path)?;

    let mut changed = false;

    // Update title
    if let Some(new_title) = title {
        task.frontmatter.title = new_title.to_string();
        changed = true;
    }

    // Update priority
    if let Some(new_priority) = priority {
        task.frontmatter.priority = new_priority;
        changed = true;
    }

    // Replace all tags
    if let Some(new_tags) = tags {
        task.frontmatter.tags = new_tags;
        changed = true;
    }

    // Add a single tag
    if let Some(tag) = add_tag {
        if !task.frontmatter.tags.contains(&tag.to_string()) {
            task.frontmatter.tags.push(tag.to_string());
            changed = true;
        }
    }

    // Remove a single tag
    if let Some(tag) = remove_tag {
        let original_len = task.frontmatter.tags.len();
        task.frontmatter.tags.retain(|t| t != tag);
        if task.frontmatter.tags.len() != original_len {
            changed = true;
        }
    }

    // Update body - check for stdin first, then explicit value
    if let Some(body_value) = body {
        if body_value == "-" {
            // Read from stdin
            if let Some(stdin_body) = read_stdin_body() {
                task.body = stdin_body;
                changed = true;
            }
        } else {
            task.body = body_value.to_string();
            changed = true;
        }
    }

    if changed {
        task.frontmatter.updated = chrono::Utc::now();
        store.save(&task, &task_path)?;
        println!("{} Updated: {} ({})", "info:".blue(), task.id(), task.title());
    } else {
        println!("{} No changes made to: {}", "info:".blue(), task.id());
    }

    Ok(())
}
