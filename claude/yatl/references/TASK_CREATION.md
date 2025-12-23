# Task Creation Guidelines

Guidance on when and how to create yatl tasks for maximum effectiveness.

## Contents

- [When to Ask First vs Create Directly](#when-to-ask)
- [Task Quality](#quality)
- [Body vs Log Usage](#body-vs-log)

## When to Ask First vs Create Directly {#when-to-ask}

### Ask the user before creating when:
- Knowledge work with fuzzy boundaries
- Task scope is unclear
- Multiple valid approaches exist
- User's intent needs clarification
- Large strategic initiatives

### Create directly when:
- Clear bug discovered during implementation
- Obvious follow-up work identified
- Technical debt with clear scope
- Dependency or blocker found
- Research tasks with specific questions

**Why ask first for knowledge work?** Task boundaries in strategic/research work are often unclear until discussed, whereas technical implementation tasks are usually well-defined. Discussion helps structure the work properly before creating tasks, preventing poorly-scoped tasks that need immediate revision.

### Examples

**Create directly (no user question needed):**
- Bug reports: `yatl new "Bug: auth doesn't check profile permissions" --tags bug`
- Research tasks: `yatl new "Research workaround for API rate limiting"`
- Technical TODOs: `yatl new "Add validation to form handler" --tags refactor`
- Side quest capture: `yatl new "Found: MCP can't read shared files" --tags bug`

**Ask first (get user input):**
- Strategic work: "Should we implement X or Y pattern?"
- Potential duplicates: Might overlap with existing work
- Large epics: Multiple approaches, unclear scope
- Major scope changes: Changing direction of existing task

**Rule of thumb:** If you can write a clear, specific task title in one sentence, create directly. If you need user input to clarify the work, ask first.

## Task Quality {#quality}

Use clear, specific titles and include sufficient context in the body to resume work later.

### Title Guidelines

**Good titles:**
- Action-oriented: "Fix login bug with special characters"
- Specific: "Add rate limiting to password reset endpoint"
- Clear scope: "Research caching strategies for API responses"

**Poor titles:**
- Vague: "Fix bug"
- Too broad: "Update database"
- No context: "Refactor code"

### Body Content

The task body should contain:
1. **Problem statement** - Why does this matter?
2. **Context** - What led to this task?
3. **Acceptance criteria** - How do we know it's done?

**Example body:**
```markdown
Users cannot log in when their password contains special characters
like & or <. This affects approximately 15% of users based on
support tickets.

Acceptance:
- Passwords with any printable ASCII character work
- Existing users can log in without password reset
- Add regression test for special character passwords
```

### Priority Guidelines

| Priority | Use When |
|----------|----------|
| `critical` | Security issues, data loss, broken production |
| `high` | Blocking other work, important features |
| `medium` | Normal priority (default) |
| `low` | Nice to have, do when convenient |

## Body vs Log Usage {#body-vs-log}

Understanding when to use body vs log entries is crucial for task clarity.

### Body (Task Description)

**Purpose:** The authoritative, current description of the task.

**Use for:**
- Problem statement (what needs to be done)
- Acceptance criteria (definition of done)
- Implementation approach (if known upfront)
- Context that won't change

**Characteristics:**
- Editable/replaceable with `yatl describe` or `yatl update --body`
- Should be the "current truth" about the task
- Read this first when resuming work

### Log Entries

**Purpose:** Append-only work history and session handoffs.

**Use for:**
- Progress updates (COMPLETED/IN_PROGRESS/NEXT)
- Key decisions made during work
- Blockers discovered
- Session handoff notes

**Characteristics:**
- Append-only with `yatl log`
- Chronological history
- Survives across sessions

### The Critical Distinction

**Body = WHAT success looks like (stable)**
```markdown
Acceptance:
- Bold and italic markdown renders correctly
- Solution accepts markdown input
- Returns document ID to caller
```

**Log = HOW we're building it (evolves)**
```
COMPLETED: Parsing logic for * and _ markers
KEY DECISION: Using two-phase approach for atomicity
IN PROGRESS: Testing edge cases for nested formatting
NEXT: Implement batch update call structure
```

### Common Mistake: Implementation in Body

**Wrong (locks you into one approach):**
```markdown
Body:
- Use JWT tokens with 1-hour expiry
- Implement with bcrypt 12 rounds
- Store in Redis
```

**Right (outcome-focused):**
```markdown
Body:
User tokens persist across sessions and refresh automatically.
Password storage is secure against rainbow table attacks.
Session data is cached for performance.
```

The right version allows implementation flexibility while the wrong version prescribes specific solutions.

### Test Yourself

If you changed the implementation approach, would the body still apply?
- **Yes** = Good body (outcome-focused)
- **No** = Move details to log entries (implementation-focused)

## Quick Reference

**Creating good tasks:**

```
Creating Task:
- [ ] Title: Clear, specific, action-oriented
- [ ] Body: Problem statement + acceptance criteria
- [ ] Priority: critical/high/medium/low
- [ ] Tags: Relevant labels (bug, feature, refactor, docs)
- [ ] Blocked-by: Any dependencies (if known)
```

**Common mistakes:**

| Mistake | Better |
|---------|--------|
| "Fix bug" | "Fix: auth token expires before refresh" |
| "Use JWT" in body | "Auth tokens persist across sessions" |
| "Update database" | "Add user_last_login column for analytics" |
| Implementation steps in body | Keep in log entries |
