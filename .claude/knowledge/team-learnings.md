# Team Learnings

Accumulated lessons and discoveries from agent work on NexusFlow. Organized by date, newest first.

---

## Template for New Entries

### YYYY-MM-DD

### [Title] ([Agent Name])
**Problem:** [What went wrong or what was discovered]

**Root Cause:** [Why it happened]

**Solution:** [How it was fixed or the pattern that works]

**Impact:** [What this enables or prevents]

**Documentation:** [Where this is documented for future reference]

---

## Common Learnings to Capture

### Svelte 5 Runes Patterns
- `$derived` must be a field assignment, never inside a getter return
- `.svelte.ts` extension required for runes in non-component files (no error, just silent failure)
- `$effect` reading and writing same `$state` creates infinite loop — use `untrack()`

### TypeScript / Build Issues
- `bun run check` must pass with 0 new errors after every change
- ~42 pre-existing warnings are known and should be ignored
- XYFlow node `data` type must be `Record<string, unknown>` — cast with `as unknown as Record<string, unknown>`

### CSS / Styling
- Each component owns its CSS scope with namespace prefixes (`.col-*`, `.env-*`, `.nf-*`, etc.)
- Dark mode: toggle `html.dark` on `document.documentElement`, not `document.body`
- `:global()` required when styling child component classes from parent `<style>` block

### Testing
- E2E selectors use `.nav-item` (not old `.toolbelt` selectors)
- Unit tests for sigv4.ts use AWS Signature Test Suite vectors
- Playwright base URL: `http://localhost:3200`

### Security
- SigV4 `hmacSha256` must check `ArrayBuffer.isView(key)` not just `key instanceof ArrayBuffer`
- Proxy has no SSRF protection yet — blocks needed for `10.x`, `172.16-31.x`, `192.168.x`, `127.x`
- Credentials must never appear in logs, error messages, or console output

---

*Add new learnings above this line, organized by date (newest first)*
