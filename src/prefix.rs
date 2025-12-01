use crate::id::TaskId;
use crate::store::{Result, Store};

/// Resolves shortest unique prefixes for task IDs.
/// Caches all task IDs for efficient batch operations.
pub struct PrefixResolver {
    all_ids: Vec<TaskId>,
}

impl PrefixResolver {
    /// Create a new resolver by loading all task IDs from the store
    pub fn new(store: &Store) -> Result<Self> {
        let all_tasks = store.list_all()?;
        let all_ids = all_tasks
            .into_iter()
            .map(|(_, t)| t.id().clone())
            .collect();
        Ok(Self { all_ids })
    }

    /// Get the shortest unique prefix for a task ID
    pub fn shortest_prefix<'a>(&self, id: &'a TaskId) -> &'a str {
        let refs: Vec<&TaskId> = self.all_ids.iter().collect();
        id.shortest_unique_prefix(&refs)
    }
}
