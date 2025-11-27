use crate::store::{Store, StoreError};
use crate::task::{Priority, Status, Task};
use colored::*;
use std::io::{self, IsTerminal, Read};
use std::path::Path;

/// Read description from stdin if it's not a TTY (i.e., piped input)
fn read_stdin_description() -> Option<String> {
    if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() && !buffer.is_empty() {
            return Some(buffer.trim().to_string());
        }
    }
    None
}

pub fn new(
    path: &Path,
    title: &str,
    priority: Option<Priority>,
    tags: Option<Vec<String>>,
    blocked_by: Option<Vec<String>>,
) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let author = store.get_author();

    let mut task = Task::new(title, author);

    // Read description from stdin if available
    if let Some(description) = read_stdin_description() {
        task.body = description;
    }

    if let Some(p) = priority {
        task.frontmatter.priority = p;
    }

    if let Some(t) = tags {
        task.frontmatter.tags = t;
    }

    // Track whether we have any unresolved blockers
    let mut has_unresolved_blockers = false;

    if let Some(blockers) = blocked_by {
        for blocker_str in blockers {
            let blocker_path = store.find(&blocker_str)?;
            let blocker = store.load(&blocker_path)?;
            task.frontmatter.blocked_by.push(blocker.id().clone());

            // Check if this blocker is unresolved (not closed or cancelled)
            if let Some(status) = store.status_from_path(&blocker_path) {
                if !matches!(status, Status::Closed | Status::Cancelled) {
                    has_unresolved_blockers = true;
                }
            }
        }
    }

    let mut task_path = store.create(&task)?;

    // If there are unresolved blockers, move to blocked/
    if has_unresolved_blockers {
        task_path = store.move_to_status(&task_path, Status::Blocked)?;
    }

    println!("{}", task.id());
    println!(
        "{} Created: {}",
        "info:".blue(),
        task_path.display()
    );

    Ok(())
}
