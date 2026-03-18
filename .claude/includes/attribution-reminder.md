# Agent Attribution Reminder

## Why Attribution Matters

When multiple agents collaborate on a project, attribution ensures:

- **Accountability**: Clear tracking of who did what work
- **Context**: Understanding which agent has context on specific features
- **Debugging**: Knowing which agent to ask about specific decisions
- **Collaboration**: Enabling effective handoffs between agents

## Attribution Format

Use this format consistently:

```
**[AGENT_NAME]** - [Action/Status]

[Details]
```

## When to Add Attribution

| Context | Attribution Required | Format Example |
|---------|---------------------|----------------|
| **PR description** | YES | `**PR Created by:** Claude Code Agent (frontend-engineer)` |
| **PR comments** | YES | `**frontend-engineer** - Addressed review feedback.\n\n[Changes]` |
| **Git commits** | YES | Include `Co-Authored-By:` in commit footer |
| **Code comments** | NO | Only add if explaining non-obvious agent decisions |
| **Planning docs** | YES | `*Prepared by distinguished-engineer - YYYY-MM-DD*` |

## Examples

### PR Description
```markdown
## Summary
[Changes description]

---
**PR Created by:** Claude Code Agent (frontend-engineer)
```

### Planning Document Footer
```markdown
---
*Prepared by distinguished-engineer - March 13, 2026*
```

## Enforcement

All slash commands that create external artifacts should include attribution:
- `/create-pr` - PR creation
- `/review-pr` - PR review comments
- `/review-security` - Security review comments
- `/update-confluence` - Confluence page updates (if used)
