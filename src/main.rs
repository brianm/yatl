use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};
use std::process;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

mod commands;
mod config;
mod id;
mod prefix;
mod store;
mod task;

use task::Priority;

#[derive(Parser)]
#[command(name = "bt")]
#[command(about = "Brian's Tasks - a minimal, file-based task tracker")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize task tracking in the current directory
    Init,

    /// Create a new task
    New {
        /// Task title
        title: String,

        /// Priority (low, medium, high, critical)
        #[arg(short, long)]
        priority: Option<String>,

        /// Comma-separated tags
        #[arg(short, long)]
        tags: Option<String>,

        /// Comma-separated IDs of blocking tasks
        #[arg(short, long)]
        blocked_by: Option<String>,
    },

    /// List tasks
    #[command(alias = "ls")]
    List {
        /// Include closed tasks
        #[arg(short, long)]
        all: bool,

        /// Long format output
        #[arg(short, long)]
        long: bool,

        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Search in title and body
        #[arg(long)]
        search: Option<String>,

        /// Limit number of results
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Show body preview (first line)
        #[arg(short, long)]
        body: bool,
    },

    /// Show task details
    Show {
        /// Task ID or prefix
        id: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show full context for working on a task
    Context {
        /// Task ID or prefix
        id: String,
    },

    /// Edit task in $EDITOR
    Edit {
        /// Task ID or prefix
        id: String,
    },

    /// Close one or more tasks
    Close {
        /// Task ID(s) or prefix(es)
        ids: Vec<String>,

        /// Reason for closing
        #[arg(short, long)]
        reason: Option<String>,
    },

    /// Reopen closed tasks
    Reopen {
        /// Task ID(s) or prefix(es)
        ids: Vec<String>,
    },

    /// Start working on tasks (open -> in-progress)
    Start {
        /// Task ID(s) or prefix(es)
        ids: Vec<String>,
    },

    /// Stop working on tasks (in-progress -> open)
    Stop {
        /// Task ID(s) or prefix(es)
        ids: Vec<String>,
    },

    /// List tasks ready to work on (no blockers)
    Ready,

    /// Suggest the highest priority ready task
    Next,

    /// Show recent activity across all tasks
    Activity {
        /// Maximum number of entries to show
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,

        /// Include closed/cancelled tasks
        #[arg(short, long)]
        all: bool,
    },

    /// Show dependency tree of active tasks
    Tree,

    /// Add a log entry to a task
    Log {
        /// Task ID or prefix
        id: String,

        /// Log message
        message: Vec<String>,
    },

    /// Mark a task as blocked by another
    Block {
        /// Task ID to block
        id: String,

        /// Task ID that blocks it
        by: String,
    },

    /// Remove a blocker from a task
    Unblock {
        /// Task ID to unblock
        id: String,

        /// Task ID to remove as blocker
        blocker: String,
    },

    /// Import tasks from a YAML file
    Import {
        /// Path to YAML file with task definitions
        file: String,
    },

    /// Update task fields programmatically
    Update {
        /// Task ID or prefix
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New priority (low, medium, high, critical)
        #[arg(long)]
        priority: Option<String>,

        /// Replace all tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// Add a single tag
        #[arg(long)]
        add_tag: Option<String>,

        /// Remove a single tag
        #[arg(long)]
        remove_tag: Option<String>,

        /// New body/description (use "-" to read from stdin)
        #[arg(long)]
        body: Option<String>,
    },

    /// Set or replace the task body/description
    Describe {
        /// Task ID or prefix
        id: String,

        /// Description text (use "-" to read from stdin)
        description: Vec<String>,
    },
}

/// VCS directory markers that indicate a repository boundary
const VCS_MARKERS: &[&str] = &[".git", ".jj", ".hg", ".svn"];

/// Find the root directory containing .tasks by walking up the directory tree.
/// Stops at VCS boundaries or permission boundaries.
fn find_tasks_root(start: &Path) -> Result<PathBuf, String> {
    let mut current = start.to_path_buf();

    loop {
        // Check for .tasks directory
        if current.join(".tasks").is_dir() {
            return Ok(current);
        }

        // Check for VCS boundary - stop here even if no .tasks found
        for marker in VCS_MARKERS {
            if current.join(marker).exists() {
                return Err(format!(
                    "Not a bt-enabled directory (found {} but no .tasks). Run 'bt init' first.",
                    marker
                ));
            }
        }

        // Try to move to parent
        let parent = match current.parent() {
            Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
            _ => return Err("Not a bt-enabled directory. Run 'bt init' first.".to_string()),
        };

        // Check if we can access the parent (permission boundary)
        #[cfg(unix)]
        {
            if let Ok(meta) = parent.metadata() {
                // Check execute permission (needed to traverse into directory)
                if meta.mode() & 0o111 == 0 {
                    return Err("Not a bt-enabled directory. Run 'bt init' first.".to_string());
                }
            } else {
                return Err("Not a bt-enabled directory. Run 'bt init' first.".to_string());
            }
        }

        #[cfg(not(unix))]
        {
            if parent.metadata().is_err() {
                return Err("Not a bt-enabled directory. Run 'bt init' first.".to_string());
            }
        }

        current = parent;
    }
}

fn main() {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Init uses cwd directly; all other commands find .tasks by walking up
    let result = match cli.command {
        Commands::Init => commands::init(&cwd),

        _ => {
            // Find .tasks root by walking up the directory tree
            let root = match find_tasks_root(&cwd) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{} {}", "error:".red(), e);
                    process::exit(1);
                }
            };

            match cli.command {
                Commands::Init => unreachable!(),

                Commands::New {
                    title,
                    priority,
                    tags,
                    blocked_by,
                } => {
                    let priority = priority.and_then(|p| p.parse::<Priority>().ok());
                    let tags =
                        tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());
                    let blocked_by =
                        blocked_by.map(|b| b.split(',').map(|s| s.trim().to_string()).collect());

                    commands::new(&root, &title, priority, tags, blocked_by)
                }

                Commands::List {
                    all,
                    long,
                    status,
                    priority,
                    tag,
                    search,
                    limit,
                    json,
                    body,
                } => commands::list(&root, all, long, status.as_deref(), priority.as_deref(), tag.as_deref(), search.as_deref(), limit, json, body),

                Commands::Show { id, json } => commands::show(&root, &id, json),

                Commands::Context { id } => commands::context(&root, &id),

                Commands::Edit { id } => commands::edit(&root, &id),

                Commands::Close { ids, reason } => {
                    for id in &ids {
                        if let Err(e) = commands::close(&root, id, reason.as_deref()) {
                            eprintln!("{} {}: {}", "error:".red(), id, e);
                        }
                    }
                    Ok(())
                }

                Commands::Reopen { ids } => {
                    for id in &ids {
                        if let Err(e) = commands::reopen(&root, id) {
                            eprintln!("{} {}: {}", "error:".red(), id, e);
                        }
                    }
                    Ok(())
                }

                Commands::Start { ids } => {
                    for id in &ids {
                        if let Err(e) = commands::start(&root, id) {
                            eprintln!("{} {}: {}", "error:".red(), id, e);
                        }
                    }
                    Ok(())
                }

                Commands::Stop { ids } => {
                    for id in &ids {
                        if let Err(e) = commands::stop(&root, id) {
                            eprintln!("{} {}: {}", "error:".red(), id, e);
                        }
                    }
                    Ok(())
                }

                Commands::Ready => commands::ready(&root),

                Commands::Next => commands::next(&root),

                Commands::Activity { limit, all } => commands::activity(&root, limit, all),

                Commands::Tree => commands::tree(&root),

                Commands::Log { id, message } => {
                    let message = message.join(" ");
                    commands::log(&root, &id, &message)
                }

                Commands::Block { id, by } => commands::block(&root, &id, &by),

                Commands::Unblock { id, blocker } => commands::unblock(&root, &id, &blocker),

                Commands::Import { file } => commands::import(&root, &file),

                Commands::Update {
                    id,
                    title,
                    priority,
                    tags,
                    add_tag,
                    remove_tag,
                    body,
                } => {
                    let priority = priority.and_then(|p| p.parse::<Priority>().ok());
                    let tags =
                        tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());

                    commands::update(
                        &root,
                        &id,
                        title.as_deref(),
                        priority,
                        tags,
                        add_tag.as_deref(),
                        remove_tag.as_deref(),
                        body.as_deref(),
                    )
                }

                Commands::Describe { id, description } => {
                    let body = if description.len() == 1 && description[0] == "-" {
                        "-".to_string()
                    } else {
                        description.join(" ")
                    };
                    commands::update(
                        &root,
                        &id,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(body.as_str()),
                    )
                }
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red(), e);
        process::exit(1);
    }
}
