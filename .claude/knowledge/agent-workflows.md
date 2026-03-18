# Agent Workflows

Standard workflows for agent collaboration on NexusFlow.

## Overview: Work Lifecycle

```
                      PROJECT OWNER (Director)
                              |
        +---------------------+---------------------+
        v                     v                     v
  +-----------+        +------------+         +-----------+
  | ARCHITECT |        | DEVELOPER  |         |    QA     |
  | Strategy  |        | Features   |         |  Testing  |
  +-----------+        +-----+------+         +-----------+
                             |
                    +--------+--------+
                    |   CODE REVIEW   |
                    +-----------------+
```

## Workflow 1: Feature Development

### Purpose
Implement new features or enhancements in NexusFlow.

### Process

| Step | Actor | Action |
|------|-------|--------|
| 1 | Owner | Describes the feature or improvement needed |
| 2 | distinguished-engineer | Reviews architecture impact, suggests approach |
| 3 | frontend-engineer | Implements the feature (SvelteKit 5 / Svelte 5) |
| 4 | frontend-engineer | Runs `bun run check` — 0 new errors |
| 5 | senior-qa-engineer | Writes/updates Vitest + Playwright tests |
| 6 | code-reviewer | Reviews code quality and security |
| 7 | Owner | Reviews and approves via `/create-pr` |

---

## Workflow 2: Bug Fix

### Process

| Step | Actor | Action |
|------|-------|--------|
| 1 | Owner | Reports the bug or failing behavior |
| 2 | frontend-engineer | Diagnoses root cause |
| 3 | frontend-engineer | Implements fix |
| 4 | frontend-engineer | Runs `bun run check` — 0 new errors |
| 5 | senior-qa-engineer | Adds regression test |
| 6 | Owner | Reviews via `/create-pr` |

---

## Workflow 3: Architecture Review

### Purpose
Evaluate design decisions before major changes.

### Process

| Step | Actor | Action |
|------|-------|--------|
| 1 | Owner | Describes the proposed change |
| 2 | distinguished-engineer | Evaluates architecture impact |
| 3 | cloud-architect | Reviews cloud/infrastructure implications |
| 4 | distinguished-engineer | Documents decision in `docs/planning/` |
| 5 | Owner | Approves approach |

---

## Workflow 4: PR Review

### Purpose
Thorough code review before merging.

### Process

| Step | Actor | Action |
|------|-------|--------|
| 1 | Developer | Completes implementation |
| 2 | Developer | Runs `/create-pr` to push and create PR |
| 3 | Owner | Runs `/review-pr {NUMBER}` for comprehensive review |
| 4 | code-reviewer | Code quality, patterns, DRY |
| 5 | security reviewer | `/review-security` — OWASP, proxy safety |
| 6 | senior-qa-engineer | Test coverage assessment |
| 7 | Developer | Addresses feedback (if needed) |
| 8 | Owner | Approves and merges via GitHub UI |

---

## Workflow 5: Testing

### Purpose
Ensure quality before merging.

### Process

| Step | Actor | Action |
|------|-------|--------|
| 1 | senior-qa-engineer | Writes Vitest unit tests for stores/utils |
| 2 | senior-qa-engineer | Writes Playwright E2E tests for user flows |
| 3 | senior-qa-engineer | Runs `bun run test:unit` and `bun run test:e2e` |
| 4 | senior-qa-engineer | Reports coverage and gaps |

---

## Agent Handoff Pattern

When work transitions between agents:

1. **Current agent** completes their portion and summarizes what was done
2. **Owner** summons the next agent via `/summon {agent-name}`
3. **Next agent** reviews recent changes and continues from the handoff point

### Handoff Summary Format

```
Completed:
- [List of what was done]

Next steps for {next-agent}:
- [What needs to happen next]

Files changed:
- [List of modified files]
```

---

## Quick Reference: Common Agent Sequences

```
New Feature:    distinguished-engineer → frontend-engineer → senior-qa-engineer → code-reviewer
Bug Fix:        frontend-engineer → senior-qa-engineer
UI/UX Change:   ux-designer → frontend-engineer → senior-qa-engineer
Architecture:   distinguished-engineer → cloud-architect
Security Audit: code-reviewer → /review-security
```

---

## Attribution Requirement

**All agent handoffs should include attribution** (per project policy):

```
**[Agent Name]** - [Action taken]

[Details of work completed or handoff context]
```

See `.claude/includes/attribution-reminder.md` for full policy.
