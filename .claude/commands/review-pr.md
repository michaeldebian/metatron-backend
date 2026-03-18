---
description: Perform a comprehensive code review using all specialist roles running in parallel
argument-hint: "PR-NUMBER"
---

# Comprehensive PR Review

Perform a comprehensive code review using all specialist roles running in parallel for thorough analysis.

## Usage

`/review-pr {PR_NUMBER}`

## Instructions

### 1. Fetch PR Details from GitHub

```bash
gh pr view {PR_NUMBER} --json title,body,files,additions,deletions,baseRefName,headRefName
gh pr diff {PR_NUMBER}
```

### 2. Launch Parallel Sub-Agents

Use the Agent tool to launch parallel sub-agents for each specialist role.

### 3. Aggregate Results

When all sub-agents complete:
- Aggregate all results and display consolidated review output
- Post consolidated summary as a PR comment via `gh pr comment`

## Parallel Sub-Agent Execution

Launch these review agents in parallel:

### Agent 1: Code Quality Review
- Review code structure, naming, patterns, DRY violations
- Check Svelte 5 runes usage (no `$derived` in getters, `.svelte.ts` extension)
- Verify TypeScript strict mode compliance
- Score: X.X/10

### Agent 2: Security Review
Execute: `/review-security {PR_NUMBER}`

### Agent 3: Performance Review
- Check for unnecessary re-renders, missing `untrack()`
- Web Worker usage for heavy computation
- Bundle size impact
- Memory leaks (event listeners, intervals without cleanup)
- Score: X.X/10

### Agent 4: UX/Accessibility Review
- Verify text labels on navigation (not icon-only)
- Check dark mode support for new UI elements
- Verify one primary CTA per view
- a11y: proper ARIA labels, keyboard navigation
- Score: X.X/10

### Agent 5: Test Coverage Review
- Check if new code has corresponding tests
- Verify Vitest unit tests for stores/utils
- Verify Playwright E2E coverage for user flows
- Score: X.X/10

## Consolidated Review Output Format

**PR Review Summary for PR #{PR_NUMBER}**

**Overall Score: X.X/10** *(Average of all specialist scores)*

**Individual Specialist Scores:**
- Code Quality: X.X/10
- Security: X.X/10
- Performance: X.X/10
- UX/Accessibility: X.X/10
- Test Coverage: X.X/10

**Critical Issues Summary:**
- Total critical issues found: X

**Top Priority Issues:**
1. [Highest priority issue]
2. [Second highest priority issue]
3. [Third highest priority issue]

**Specialist Recommendations:**
- **Code Quality:** [Key recommendations]
- **Security:** [Key recommendations]
- **Performance:** [Key recommendations]
- **UX/a11y:** [Key recommendations]
- **Testing:** [Key recommendations]

**Overall Recommendation:** [APPROVED/APPROVED WITH CHANGES/CHANGES REQUIRED]
