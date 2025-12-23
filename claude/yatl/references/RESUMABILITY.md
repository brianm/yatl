# Making Tasks Resumable Across Sessions

Guide for writing task bodies and log entries that enable seamless work resumption.

## When Resumability Matters

**Use enhanced documentation for:**
- Multi-session technical features with API integration
- Complex algorithms requiring code examples
- Features with specific output format requirements
- Work with undocumented or "occult" API capabilities
- Tasks that might be picked up after weeks away

**Skip for:**
- Simple bug fixes with clear scope
- Well-understood patterns (CRUD operations, etc.)
- Single-session tasks
- Work with obvious acceptance criteria

**The test:** Would a fresh Claude instance (or you after 2 weeks) struggle to resume this work from the task alone? If yes, add implementation details.

## Anatomy of a Resumable Task

### Minimal (Always Include)

```markdown
Body:
What needs to be built and why.
Acceptance criteria: concrete, testable outcomes.
```

### Enhanced (Complex Technical Work)

For complex features, enhance log entries with implementation guides:

```markdown
yatl log <id> "IMPLEMENTATION GUIDE:

WORKING CODE:
service = build('drive', 'v3', credentials=creds)
result = service.about().get(fields='importFormats').execute()
Returns: dict with 49 entries like {'text/markdown': [...]}

DESIRED OUTPUT FORMAT:
# Drive Import Formats (markdown with categorized list)

RESEARCH CONTEXT:
text/markdown support added July 2024, not in static docs.
This is why dynamic queries matter."
```

## What to Include

### For API Integration Work

```markdown
WORKING CODE (tested):
```python
# Paste actual code that works
# Include imports and setup
# Show what it returns
```

API RESPONSE SAMPLE:
Shows actual data structure (not docs description)
- field_a: "example value"
- field_b: [list of items]

KEY DISCOVERY:
What did you learn that isn't in the docs?
```

### For Algorithm/Logic Work

```markdown
ALGORITHM APPROACH:
1. First, we parse X to find Y
2. Then transform using Z pattern
3. Edge cases: handle A, B, C specially

TESTED INPUTS:
- Input: "example" -> Output: "result"
- Edge case: "special" -> Output: "handled"

WHY THIS APPROACH:
Considered alternatives X, Y. Chose Z because...
```

### For Output Format Work

```markdown
DESIRED OUTPUT FORMAT:
```
# Exact Example

Not just "return markdown" but actual structure:
- Section headers like this
- Bullet formatting like this
- Code blocks formatted like this
```

WHY THIS FORMAT:
User requested / matches existing style / etc.
```

## Real Example: Before vs After

### Not Resumable

```
Title: Add dynamic capabilities resources
Body: Query Google APIs for capabilities and return as resources
```

**Problem:** Future Claude doesn't know:
- Which API endpoints to call
- What the responses look like
- What format to return

### Resumable

```
Title: Add dynamic capabilities resources

Body:
Query Google APIs for system capabilities (import formats, themes, quotas)
that aren't in static docs. Makes server self-documenting.

Acceptance:
- User queries capabilities endpoint
- Response shows all supported formats
- Output is readable markdown, not raw JSON
- Queries live API (not static data)

---
## Log

### 2025-01-15T10:30:00Z claude

IMPLEMENTATION GUIDE:

WORKING CODE (tested):
from googleapiclient.discovery import build
service = build('drive', 'v3', credentials=creds)
about = service.about().get(
    fields='importFormats,exportFormats,folderColorPalette'
).execute()

Returns:
- importFormats: dict, 49 entries like {'text/markdown': [...]}
- exportFormats: dict, 10 entries
- folderColorPalette: list, 24 hex strings

OUTPUT FORMAT EXAMPLE:
# Drive Import Formats

Google Drive supports 49 import formats:

## Text Formats
- **text/markdown** -> Google Docs (NEW July 2024)
- text/plain -> Google Docs

RESEARCH CONTEXT:
text/markdown support announced July 2024 but NOT in static Google docs.
This is why dynamic resources matter - static docs are outdated.
```

**Result:** Fresh Claude instance can:
1. See working API query code
2. Understand response structure
3. Know desired output format
4. Implement with full context

## Log Entry Patterns

### Session Handoff Pattern

```bash
yatl log <id> "COMPLETED: Parsed markdown into structured format.
IN PROGRESS: Implementing API insertion.
NEXT: Debug batchUpdate call - getting 400 error on formatting.
KEY DECISION: Using two-phase approach (insert text, then format) based on reference implementation."
```

### Discovery Pattern

```bash
yatl log <id> "DISCOVERY: The API supports text/markdown as of July 2024.
This is NOT documented in the official guides.
Tested with: service.about().get(fields='importFormats')
Confirmed: 'text/markdown' in result['importFormats']"
```

### Blocker Pattern

```bash
yatl log <id> "BLOCKER: Rate limiting prevents testing at scale.
Attempted: 100 requests -> 429 after 50
Need: User input on whether to add delays or use batch API
Workaround tried: Exponential backoff (still hitting limits)"
```

## When to Add Implementation Details

**During task creation:**
- Already have working code from research? Include it.
- Clear output format in mind? Show example.

**During work (update log):**
- Just got API query working? Add to log.
- Discovered important context? Document it.
- Made key decision? Explain rationale.

**At session end:**
- If resuming will be hard, add implementation guide.
- If obvious, skip it.

**The principle:** Help your future self (or next Claude) resume without rediscovering everything.

## Anti-Patterns

### Over-Documenting Simple Work

```markdown
Title: Fix typo in README

Log: IMPLEMENTATION GUIDE
WORKING CODE: Open README.md, change "teh" to "the"...
```

**Problem:** Wastes tokens on obvious work.

### Raw Data Dumps

```markdown
Log: API RESPONSE:
{giant unformatted JSON blob spanning 100 lines}
```

**Problem:** Hard to read. Extract relevant parts, show structure.

### Vague Progress Notes

```markdown
Log: Working on auth. Made some progress. More to do.
```

**Problem:** Zero useful information for resumption.

### Right Balance

```markdown
Log: COMPLETED: JWT token generation (RS256, 1hr expiry).
KEY DECISION: RS256 over HS256 for key rotation support.
API tested: POST /auth/token returns {access_token, refresh_token, expires_in}.
IN PROGRESS: Refresh token endpoint.
NEXT: Add rate limiting (5 attempts per 15 min).
```

## Quick Reference

**For complex technical tasks, log entries should answer:**

1. What code/queries work? (tested, not theoretical)
2. What do responses look like? (actual structure)
3. What format should output have? (show, don't describe)
4. What context matters? (discoveries, decisions, rationale)

**Self-test before ending session:**

"If I read only this task file in 2 weeks, could I continue without asking questions or re-researching?"

- **Yes** = Good resumability
- **No** = Add more implementation details
