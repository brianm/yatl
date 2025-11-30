---
name: bt-tasks
description: Track multi-session work with dependencies using bt (Brian's Tasks). Use when work spans multiple sessions, has dependencies, or requires persistent context across compaction cycles. For simple single-session linear tasks, TodoWrite remains appropriate.
---

# bt Task Tracking

## Overview

bt is a minimal, file-based task tracker that lives in git repositories. Tasks are human-readable markdown files with YAML frontmatter, organized by status in directories. Use for multi-session work with dependencies; use TodoWrite for simple single-session tasks.

**Key principle**: Status lives in the directory, not the file. Tasks in `.tasks/open/` are open, tasks in `.tasks/in-progress/` are in progress, etc.

## When to Use bt vs TodoWrite

### Use bt when:
- **Multi-session work** - Tasks spanning multiple compaction cycles or days
- **Complex dependencies** - Work with blockers or prerequisites
- **Knowledge work** - Strategic documents, research, or tasks with fuzzy boundaries
- **Side quests** - Exploratory work that might pause the main task
- **Project memory** - Need to resume work after weeks away with full context

### Use TodoWrite when:
- **Single-session tasks** - Work that completes within current session
- **Linear execution** - Straightforward step-by-step tasks with no branching
- **Immediate context** - All information already in conversation
- **Simple tracking** - Just need a checklist to show progress

**Key insight**: If resuming work after 2 weeks would be difficult without bt, use bt. If the work can be picked up from a markdown skim, TodoWrite is sufficient.

### Test Yourself: bt or TodoWrite?

**Choose bt if:**
- "Will I need this context in 2 weeks?" -> Yes = bt
- "Could conversation history get compacted?" -> Yes = bt
- "Does this have blockers/dependencies?" -> Yes = bt
- "Is this fuzzy/exploratory work?" -> Yes = bt

**Choose TodoWrite if:**
- "Will this be done in this session?" -> Yes = TodoWrite
- "Is this just a task list for me right now?" -> Yes = TodoWrite
- "Is this linear with no branching?" -> Yes = TodoWrite

**When in doubt**: Use bt. Better to have persistent memory you don't need than to lose context you needed.

**For detailed decision criteria and examples, read:** [references/BOUNDARIES.md](references/BOUNDARIES.md)

## Surviving Compaction Events

**Critical**: Compaction events delete conversation history but preserve bt tasks. After compaction, bt tasks are your only persistent memory.

**What survives compaction:**
- All task files (title, body, log entries)
- Complete work history in log sections
- Dependency relationships

**What doesn't survive:**
- Conversation history
- TodoWrite lists
- Recent discussion context

**Writing log entries for post-compaction recovery:**

Write log entries as if explaining to a future agent with zero conversation context:

**Pattern:**
```markdown
## Log

### 2025-01-15T10:30:00Z brian

COMPLETED: Implemented JWT refresh endpoint with rate limiting
IN PROGRESS: Testing password reset flow
BLOCKERS: Need user input on email template design
KEY DECISIONS: Using RS256 over HS256 for key rotation support
NEXT: Add rate limiting (5 attempts/15min) once email template decided
```

**After compaction:** `bt show <task-id>` reconstructs full context from log entries.

### Log Entry Quality Self-Check

Before checkpointing (especially pre-compaction), verify your log entries pass these tests:

**Future-me test**: "Could I resume this work in 2 weeks with zero conversation history?"
- [ ] What was completed? (Specific deliverables, not "made progress")
- [ ] What's in progress? (Current state + immediate next step)
- [ ] What's blocked? (Specific blockers with context)
- [ ] What decisions were made? (Why, not just what)

**Good log entry example:**
```
COMPLETED: JWT auth with RS256 (1hr access, 7d refresh tokens)
KEY DECISION: RS256 over HS256 per security review - enables key rotation
IN PROGRESS: Password reset flow - email service working, need rate limiting
BLOCKERS: Waiting on user decision: reset token expiry (15min vs 1hr trade-off)
NEXT: Implement rate limiting (5 attempts/15min) once expiry decided
```

**Bad log entry example:**
```
Working on auth. Made some progress. More to do.
```

**For complete compaction recovery workflow, read:** [references/WORKFLOWS.md](references/WORKFLOWS.md#compaction-survival)

## Session Start Protocol

**bt is available when:**
- Project has a `.tasks/` directory

**At session start, always check for bt availability and run ready check.**

### Session Start Checklist

Copy this checklist when starting any session where bt is available:

```
Session Start:
- [ ] Run bt next to get suggested task
- [ ] Run bt list --status in-progress for active work
- [ ] If in_progress exists: bt context <task-id> for full context
- [ ] If starting new work: bt context <suggested-id> then bt start <id>
- [ ] Report context to user: "Suggested: [task]. In progress: [task]"
```

**Efficient pattern using new commands:**
```bash
bt next                              # Highest priority ready task
bt list --status in-progress -n 3    # Check for active work
bt context <id>                      # Load full context (body + blockers + log)
```

**Report format**:
- "Suggested task: [title] (high priority). Should I start on this?"
- "Task Y is in-progress. Last session: [from context log]. Should I continue?"

This establishes immediate shared context about available and active work without requiring user prompting.

**For detailed session start workflows, read:** [references/WORKFLOWS.md](references/WORKFLOWS.md#session-start)

---

## Progress Checkpointing

Update bt task logs at these checkpoints (don't wait for session end):

**Critical triggers:**
- **Context running low** - User says "running out of context" / "approaching compaction"
- **Token budget > 70%** - Proactively checkpoint when approaching limits
- **Major milestone reached** - Completed significant piece of work
- **Hit a blocker** - Can't proceed, need to capture what was tried
- **Task transition** - Switching tasks or about to close this one
- **Before user input** - About to ask decision that might change direction

**Checkpoint command:**
```bash
bt log <task-id> "COMPLETED: ... IN PROGRESS: ... NEXT: ..."
```

**Checkpoint checklist:**
```
Progress Checkpoint:
- [ ] Add log entry with COMPLETED/IN_PROGRESS/NEXT format
- [ ] Document KEY DECISIONS or BLOCKERS since last update
- [ ] If discovered new work: create task with bt new
- [ ] Verify log is self-explanatory for post-compaction resume
```

**Most important**: When user says "running out of context" OR when you see >70% token usage - checkpoint immediately, even if mid-task.

---

## AI-Optimized Commands

These commands are designed for efficient AI assistant workflows:

### Quick Context Loading

```bash
# Get suggested next task (highest priority ready)
bt next

# Full context for a specific task (includes blockers, blocks, recent log)
bt context <id>

# See recent activity across all tasks
bt activity -n 10
```

### Efficient Searching

```bash
# Search with body preview to understand tasks quickly
bt list --body --search "authentication"

# Filter by tag with limited output
bt list --tag bug -n 5 --body

# JSON output for programmatic parsing
bt list --json
bt show <id> --json
```

### Batch Operations

```bash
# Close multiple completed tasks at once
bt close a1b2 c3d4 e5f6 --reason "Sprint complete"

# Import multiple tasks with dependencies from YAML
bt import tasks.yaml
```

### Visualizing Dependencies

```bash
# See full dependency tree with ready/blocked colors
bt tree
```

**Recommended session start:**
```bash
bt next                     # What should I work on?
bt context <id>             # Load full context for that task
bt start <id>               # Mark as in-progress
```

---

## Core Operations

### Essential Operations

**Check ready work:**
```bash
bt ready                    # Tasks with no unresolved blockers
bt next                     # Highest priority ready task (recommended)
```

**Create new task:**
```bash
bt new "Task title"
bt new "Task title" --priority high --tags bug,auth
bt new "Task title" --blocked-by task1,task2

# With piped description
echo "Detailed description" | bt new "Fix login bug"
```

**View task details:**
```bash
bt show a1b2               # Prefix matching works
bt show a1b2c3d4           # Full ID also works
```

**Start working on task:**
```bash
bt start a1b2              # Moves open -> in-progress
```

**Stop working (pause):**
```bash
bt stop a1b2               # Moves in-progress -> open
```

**Add log entry:**
```bash
bt log a1b2 "Found root cause in auth.py"
bt log a1b2 "COMPLETED: JWT validation" "NEXT: Add tests"
```

**Close completed task:**
```bash
bt close a1b2
bt close a1b2 --reason "Fixed in commit abc123"
```

**Reopen a closed task:**
```bash
bt reopen a1b2             # Moves closed -> open
```

**List tasks:**
```bash
bt list                    # Active tasks (open, in-progress, blocked)
bt list --all              # All tasks including closed
bt list --status open      # Filter by status
bt list --priority high    # Filter by priority
```

**Update task fields:**
```bash
bt update a1b2 --title "New title"
bt update a1b2 --priority critical
bt update a1b2 --add-tag documentation
```

**Set task description/body:**
```bash
bt describe a1b2 "Full description of the task"
bt describe a1b2 -    # Read from stdin
echo "Description" | bt describe a1b2 -

# Alternative using update command:
bt update a1b2 --body "Description text"
bt update a1b2 --body -    # Read from stdin
```

**When to use body vs log:**
- **Body**: The authoritative, current description (editable, replaceable)
- **Log**: Work history, progress notes, decisions (append-only)

Use `bt describe` (or `bt update --body`) for implementation plans, acceptance criteria, or any content that should be the "current truth". Use `bt log` for progress updates and historical notes.

**For complete CLI reference with all flags and examples, read:** [references/CLI_REFERENCE.md](references/CLI_REFERENCE.md)

## Dependency Management

bt supports automatic blocking and unblocking based on dependencies.

**Add a blocker:**
```bash
bt block <task-to-block> <blocker-task>
bt block c3d4 a1b2         # c3d4 is blocked by a1b2
```

When you add a blocker, the task automatically moves to `.tasks/blocked/`.

**Remove a blocker:**
```bash
bt unblock c3d4 a1b2       # Remove blocker relationship
```

**Automatic unblocking**: When you close a blocking task, all tasks it was blocking are automatically moved from `blocked/` to `open/` (if they have no other blockers).

**View blockers:**
```bash
bt show c3d4               # Shows blocked_by list in frontmatter
```

---

## Task Lifecycle Workflow

### 1. Discovery Phase (Proactive Task Creation)

**During exploration or implementation, proactively create tasks for:**
- Bugs or problems discovered
- Potential improvements noticed
- Follow-up work identified
- Technical debt encountered
- Questions requiring research

**Pattern:**
```bash
# When encountering new work during a task:
bt new "Found: auth doesn't handle profile permissions" --tags bug

# Continue with original task - new task persists for later
```

**Key benefit**: Capture context immediately instead of losing it when conversation ends.

### 2. Execution Phase (Status Maintenance)

**Mark tasks in_progress when starting work:**
```bash
bt start a1b2
```

**Add log entries throughout work:**
```bash
bt log a1b2 "Researched OAuth2 providers - recommending Auth0"
bt log a1b2 "Implemented authorization flow"
```

**Close when complete:**
```bash
bt close a1b2 --reason "Implemented JWT validation with tests passing"
```

### 3. Planning Phase (Dependency Graphs)

For complex multi-step work, structure tasks with dependencies before starting:

**Create tasks:**
```bash
bt new "Build API"          # -> a1b2
bt new "Write API tests"    # -> c3d4
```

**Link with dependencies:**
```bash
bt block c3d4 a1b2          # c3d4 blocked by a1b2
```

**Work the graph:**
```bash
bt ready                    # Shows only a1b2 (c3d4 is blocked)
bt start a1b2
# ... do work ...
bt close a1b2               # c3d4 automatically unblocks!
bt ready                    # Now shows c3d4
```

---

## Integration with TodoWrite

**Both tools complement each other at different timescales:**

### Temporal Layering Pattern

**TodoWrite** (short-term working memory - this hour):
- Tactical execution: "Review Section 3", "Expand Q&A answers"
- Marked completed as you go
- Present/future tense ("Review", "Expand", "Create")
- Ephemeral: Disappears when session ends

**bt** (long-term episodic memory - this week/month):
- Strategic objectives: "Continue work on strategic planning document"
- Key decisions and outcomes in log entries
- Past tense in logs ("COMPLETED", "Discovered", "Blocked by")
- Persistent: Survives compaction and session boundaries

### The Handoff Pattern

1. **Session start**: Read bt task -> Create TodoWrite items for immediate actions
2. **During work**: Mark TodoWrite items completed as you go
3. **Reach milestone**: Add bt log entry with outcomes + context
4. **Session end**: TodoWrite disappears, bt task survives with enriched log

**After compaction**: TodoWrite is gone forever, but bt log entries reconstruct what happened.

### Example: TodoWrite tracks execution, bt captures meaning

**TodoWrite:**
```
[completed] Implement login endpoint
[in_progress] Add password hashing with bcrypt
[pending] Create session middleware
```

**Corresponding bt log entry:**
```bash
bt log a1b2 "COMPLETED: Login endpoint with bcrypt password hashing (12 rounds). KEY DECISION: Using JWT tokens (not sessions) for stateless auth - simplifies horizontal scaling. IN PROGRESS: Session middleware implementation. NEXT: Need user input on token expiry time (1hr vs 24hr trade-off)."
```

**Don't duplicate**: TodoWrite tracks execution, bt captures meaning and context.

---

## Common Patterns

### Pattern 1: Knowledge Work Session

**Scenario**: User asks "Help me write a proposal for expanding the analytics platform"

**What you see:**
```bash
$ bt ready
a1b2 high Research analytics platform expansion proposal

$ bt show a1b2
# Shows log with previous session context
```

**What you do:**
1. Read log to understand current state
2. Create TodoWrite for immediate work
3. Work on tasks, mark TodoWrite items completed
4. At milestone, add bt log entry
5. Continue or close task when done

### Pattern 2: Side Quest Handling

During main task, discover a problem:
1. Create task: `bt new "Found: inventory system needs refactoring" --tags refactor`
2. Assess: blocker or can defer?
3. If blocker: `bt block main-task new-task`, work on new task
4. If deferrable: note in log, continue main task

### Pattern 3: Multi-Session Project Resume

Starting work after time away:
1. Run `bt ready` to see available work
2. Run `bt list --status in-progress` to see active work
3. Run `bt show <task-id>` to read full context from log
4. Start or continue work

**For complete workflow walkthroughs with checklists, read:** [references/WORKFLOWS.md](references/WORKFLOWS.md)

---

## Task Creation Guidelines

**Quick guidelines:**
- Ask user first for knowledge work with fuzzy boundaries
- Create directly for clear bugs, technical debt, or discovered work
- Use clear titles, sufficient context in body
- Use priority levels: low, medium, high, critical
- Use tags for categorization

### Task Creation Checklist

```
Creating Task:
- [ ] Title: Clear, specific, action-oriented
- [ ] Body: Problem statement (WHY this matters)
- [ ] Priority: low/medium/high/critical
- [ ] Tags: Relevant labels (bug, feature, refactor, docs, etc.)
- [ ] Blocked-by: Any dependencies (optional)
```

**For detailed guidance on when to ask vs create, read:** [references/BOUNDARIES.md](references/BOUNDARIES.md)

---

## Data Storage

bt stores tasks in `.tasks/` organized by status:

```
.tasks/
├── config.yaml       # Project configuration (default author)
├── .gitattributes    # Git merge strategy (union merge)
├── open/             # Ready to work on
├── in-progress/      # Currently being worked on
├── blocked/          # Waiting on dependencies
├── closed/           # Completed successfully
└── cancelled/        # Will not be done
```

### Task File Format

```markdown
---
title: Fix login bug with special characters
id: a1b2c3d4
created: 2025-11-25T10:30:45Z
updated: 2025-11-25T14:22:00Z
author: brian
priority: high
tags:
  - bug
  - auth
blocked_by: []
---

Users cannot log in when password contains special characters.

---
## Log

### 2025-11-25T11:00:00Z brian

Found the root cause in legacy auth path.

### 2025-11-25T14:22:00Z brian

Fixed in commit abc1234. Need to add tests before closing.
```

**Key insight**: Task files are human-readable markdown. You can read them directly if needed.

---

## Troubleshooting

**If bt command not found:**
- Check that bt binary is in PATH
- Verify with `which bt` or `bt --help`

**If tasks seem lost:**
- Use `bt list --all` to see all tasks including closed
- Check all status directories in `.tasks/`

**If dependencies seem wrong:**
- Use `bt show <task-id>` to see blocked_by list
- Dependencies are directional: `bt block A B` means A is blocked by B

---

## Reference Files

| Reference | Read When |
|-----------|-----------|
| [references/BOUNDARIES.md](references/BOUNDARIES.md) | Need detailed decision criteria for bt vs TodoWrite |
| [references/CLI_REFERENCE.md](references/CLI_REFERENCE.md) | Need complete command reference with all flags |
| [references/WORKFLOWS.md](references/WORKFLOWS.md) | Need step-by-step workflows with checklists |
