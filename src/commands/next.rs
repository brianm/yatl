use crate::prefix::PrefixResolver;
use crate::store::{Store, StoreError};
use crate::task::Priority;
use colored::*;
use std::path::Path;

pub fn next(path: &Path) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let mut tasks = store.list_ready()?;

    if tasks.is_empty() {
        println!("{}", "No tasks ready to work on.".dimmed());
        return Ok(());
    }

    // Sort by priority (critical first) then by creation date (oldest first)
    tasks.sort_by(|(_, a), (_, b)| {
        let priority_order = |p: Priority| match p {
            Priority::Critical => 0,
            Priority::High => 1,
            Priority::Medium => 2,
            Priority::Low => 3,
        };

        let priority_cmp = priority_order(a.priority()).cmp(&priority_order(b.priority()));
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // Same priority, sort by creation date (oldest first)
        a.frontmatter.created.cmp(&b.frontmatter.created)
    });

    // Get the top task
    let (_, task) = &tasks[0];

    // Resolve shortest unique prefix across ALL tasks (including closed/cancelled)
    // This ensures displayed prefix works with `bt edit`, which searches all directories
    let resolver = PrefixResolver::new(&store)?;
    let short_id = resolver.shortest_prefix(task.id());

    let priority_colored = match task.priority() {
        Priority::Critical => "critical".red(),
        Priority::High => "high".yellow(),
        Priority::Medium => "medium".normal(),
        Priority::Low => "low".blue(),
    };

    println!("{}\t{}\t{}", short_id, priority_colored, task.title());

    // Show body preview if available
    let body_preview: String = task
        .body
        .lines()
        .take_while(|line| !line.starts_with("## Log"))
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim()
        .chars()
        .take(80)
        .collect();

    if !body_preview.is_empty() {
        println!("    {}", body_preview.dimmed());
    }

    Ok(())
}
