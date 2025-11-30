# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`bt` (Brian's Tasks) is a minimal, file-based task tracker that lives in git repositories. Written in Rust, it stores tasks as human-readable markdown files with YAML frontmatter.

## Development

**Build:**
```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo test               # Run tests
```

**Run locally:**
```bash
cargo run -- help        # Show usage
cargo run -- init        # Initialize .tasks/ in current directory
cargo run -- new "Task"  # Create a task
cargo run -- list        # List open tasks
```

**Environment variables:**
- `EDITOR` - Editor for `bt edit` command

## Architecture

### Project Structure
```
src/
├── main.rs           # CLI entry point (clap)
├── lib.rs            # Library exports
├── task.rs           # Task struct, Status, Priority enums
├── store.rs          # File-based task storage
├── id.rs             # Crockford base32 ID generation
├── config.rs         # Config file handling
└── commands/         # Command implementations
    ├── mod.rs
    ├── init.rs
    ├── new.rs
    ├── list.rs
    ├── show.rs
    ├── edit.rs
    ├── start.rs
    ├── stop.rs
    ├── close.rs
    ├── reopen.rs
    ├── ready.rs
    ├── log.rs
    ├── block.rs
    ├── unblock.rs
    ├── context.rs
    ├── activity.rs
    ├── tree.rs
    ├── next.rs
    ├── import.rs
    ├── update.rs
    └── describe.rs
```

### Data Storage
```
.tasks/
├── config.yaml       # Project configuration
├── .gitattributes    # Git merge strategy (union merge)
├── open/             # Ready to work on
├── in-progress/      # Currently being worked on
├── blocked/          # Waiting on dependencies
├── closed/           # Completed successfully
└── cancelled/        # Will not be done
```

### Task File Format
Markdown with YAML frontmatter:
- Required: `title`, `id`, `created`, `updated`
- Optional: `author`, `priority`, `tags`, `blocked_by`, `blocks`, `parent`, `children`
- Body: description + append-only `## Log` section
- **Status is NOT stored in the file** - it's derived from the directory location

### Key Design Decisions
- **Random IDs**: 8-character Crockford base32 IDs (40 bits of randomness)
- **Directory-based status**: Status is determined by directory (open/, in-progress/, blocked/, closed/, cancelled/)
- **Automatic blocking**: Tasks move to blocked/ when blockers added, back to open/ when resolved
- **Union merge**: `.gitattributes` configures `merge=union` for log sections
- **Prefix matching**: `bt show a1b2` finds tasks starting with `a1b2`
- **Task hierarchies**: Parent/child relationships via `parent` and `children` fields

### Dependencies
- `clap` - CLI parsing
- `serde` + `serde_yaml` - YAML frontmatter
- `serde_json` - JSON serialization
- `chrono` - Timestamps
- `colored` - Terminal colors
- `thiserror` - Error handling
- `glob` - File matching
- `rand` - Random bytes for IDs
