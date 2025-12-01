use crate::prefix::PrefixResolver;
use crate::store::{Store, StoreError};
use crate::task::Priority;
use colored::*;
use std::path::Path;

pub fn ready(path: &Path) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let tasks = store.list_ready()?;

    // Resolve shortest unique prefixes across ALL tasks (including closed/cancelled)
    // This ensures displayed prefixes work with `bt edit`, which searches all directories
    let resolver = PrefixResolver::new(&store)?;

    for (_, task) in &tasks {
        let short_id = resolver.shortest_prefix(task.id());

        let priority_colored = match task.priority() {
            Priority::Critical => "critical".red(),
            Priority::High => "high".yellow(),
            Priority::Medium => "medium".normal(),
            Priority::Low => "low".blue(),
        };

        println!("{}\t{}\t{}", short_id, priority_colored, task.title());
    }

    Ok(())
}
