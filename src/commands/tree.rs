use crate::id::TaskId;
use crate::prefix::PrefixResolver;
use crate::store::{Store, StoreError};
use colored::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Display a DAG of task dependencies
pub fn tree(path: &Path) -> Result<(), StoreError> {
    let store = Store::open(path)?;

    // Load all active tasks (open, in-progress, blocked)
    let active_tasks = store.list_active()?;

    if active_tasks.is_empty() {
        println!("No active tasks.");
        return Ok(());
    }

    // Resolve shortest unique prefixes across ALL tasks (including closed/cancelled)
    // This ensures displayed prefixes work with `bt edit`, which searches all directories
    let resolver = PrefixResolver::new(&store)?;

    // Build task info: id -> (title, short_id, blocked_by)
    let mut task_info: HashMap<TaskId, (String, String, Vec<TaskId>)> = HashMap::new();

    for (_, task) in &active_tasks {
        let short_id = resolver.shortest_prefix(task.id()).to_string();
        task_info.insert(
            task.id().clone(),
            (
                task.frontmatter.title.clone(),
                short_id,
                task.frontmatter.blocked_by.clone(),
            ),
        );
    }

    let active_ids: HashSet<TaskId> = active_tasks.iter().map(|(_, t)| t.id().clone()).collect();

    // Build "blocks" map: task_id -> Vec<task_ids it blocks>
    let mut blocks: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
    for (task_id, (_, _, blocked_by)) in &task_info {
        for blocker_id in blocked_by {
            if active_ids.contains(blocker_id) {
                blocks
                    .entry(blocker_id.clone())
                    .or_default()
                    .push(task_id.clone());
            }
        }
    }

    // Find roots (tasks with no active blockers)
    let mut roots: Vec<TaskId> = task_info
        .iter()
        .filter(|(_, (_, _, blocked_by))| {
            blocked_by.iter().all(|b| !active_ids.contains(b))
        })
        .map(|(id, _)| id.clone())
        .collect();

    // Sort roots by title
    roots.sort_by(|a, b| {
        let a_title = task_info.get(a).map(|(t, _, _)| t.as_str()).unwrap_or("");
        let b_title = task_info.get(b).map(|(t, _, _)| t.as_str()).unwrap_or("");
        a_title.cmp(b_title)
    });

    // Track which tasks have been printed
    let mut printed: HashSet<TaskId> = HashSet::new();

    // Print each root and its descendants
    for (i, root_id) in roots.iter().enumerate() {
        let is_last_root = i == roots.len() - 1;
        print_task_tree(
            root_id,
            &task_info,
            &blocks,
            &active_ids,
            &mut printed,
            "",
            is_last_root,
            true,
        );
    }

    Ok(())
}

fn print_task_tree(
    task_id: &TaskId,
    task_info: &HashMap<TaskId, (String, String, Vec<TaskId>)>,
    blocks: &HashMap<TaskId, Vec<TaskId>>,
    active_ids: &HashSet<TaskId>,
    printed: &mut HashSet<TaskId>,
    prefix: &str,
    is_last: bool,
    is_root: bool,
) {
    // Skip if already printed
    if printed.contains(task_id) {
        return;
    }

    let (title, short_id, blocked_by) = match task_info.get(task_id) {
        Some(info) => info,
        None => return,
    };

    // Get active blockers
    let active_blockers: Vec<&TaskId> = blocked_by
        .iter()
        .filter(|b| active_ids.contains(*b))
        .collect();

    // Only print if all blockers have been printed (ensures proper ordering)
    if !active_blockers.iter().all(|b| printed.contains(*b)) {
        return;
    }

    printed.insert(task_id.clone());

    // Determine the connector
    let connector = if is_root {
        ""
    } else if is_last {
        "└── "
    } else {
        "├── "
    };

    // Color: green if ready (no active blockers), red if blocked
    let colored_id = if active_blockers.is_empty() {
        short_id.green()
    } else {
        short_id.red()
    };

    // Show blockers annotation if multiple
    if active_blockers.len() > 1 {
        let blocker_ids: Vec<String> = active_blockers
            .iter()
            .filter_map(|b| task_info.get(*b).map(|(_, sid, _)| sid.clone()))
            .collect();
        println!(
            "{}{}{}  {}  {}",
            prefix,
            connector,
            colored_id,
            title,
            format!("(blocked by: {})", blocker_ids.join(", ")).dimmed()
        );
    } else {
        println!("{}{}{}  {}", prefix, connector, colored_id, title);
    }

    // Get and sort children (tasks this one blocks)
    let mut children: Vec<TaskId> = blocks.get(task_id).cloned().unwrap_or_default();

    // Filter to only include children where THIS task is a blocker
    // and all OTHER blockers have been printed
    children.retain(|child_id| {
        if let Some((_, _, child_blocked_by)) = task_info.get(child_id) {
            let child_active_blockers: Vec<&TaskId> = child_blocked_by
                .iter()
                .filter(|b| active_ids.contains(*b))
                .collect();

            // All blockers except this one must be printed
            child_active_blockers.iter().all(|b| {
                *b == task_id || printed.contains(*b)
            })
        } else {
            false
        }
    });

    children.sort_by(|a, b| {
        let a_title = task_info.get(a).map(|(t, _, _)| t.as_str()).unwrap_or("");
        let b_title = task_info.get(b).map(|(t, _, _)| t.as_str()).unwrap_or("");
        a_title.cmp(b_title)
    });

    // Calculate new prefix for children
    let new_prefix = if is_root {
        prefix.to_string()
    } else if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    for (i, child_id) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        print_task_tree(
            child_id,
            task_info,
            blocks,
            active_ids,
            printed,
            &new_prefix,
            is_last_child,
            false,
        );
    }
}
