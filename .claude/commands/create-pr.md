---
description: Create a pull request on GitHub with branch protection validation
---

# Create Pull Request

Create a pull request on GitHub with branch protection validation.

## Arguments

**ARGUMENTS**: Optional target branch name (defaults to `main` if not provided)

**Examples:**
- `/create-pr` -> Creates PR targeting `main` (default)
- `/create-pr main` -> Creates PR targeting `main` (explicit)
- `/create-pr develop` -> Creates PR targeting `develop`

## Instructions

1. **Parse target branch argument:**
   - If ARGUMENTS is provided and not empty, use it as the target branch
   - If ARGUMENTS is empty or not provided, default to `main`
   - Trim whitespace from the argument

2. **Validate source branch:**
   - Current branch must NOT be `main` or `master`
   - Current branch should have commits ahead of target

3. **Check current state:**
   - Run `bun run check` to verify 0 new TypeScript errors
   - Run `git status` to check for uncommitted changes
   - Warn if there are uncommitted changes

4. **Check for merge conflicts:**
   - Fetch latest from origin: `git fetch origin`
   - Check if target branch has diverged
   - If conflicts exist, stop and prompt user for resolution

5. **Push branch to origin** if not already pushed:
   ```bash
   git push -u origin HEAD
   ```

6. **Create PR using GitHub CLI:**
   ```bash
   gh pr create --base {TARGET_BRANCH} --title "{PR_TITLE}" --body "{PR_BODY}"
   ```

## PR Description Format

```markdown
## Summary

[Detailed description of changes based on commit analysis]

## Changes

- [List of key changes from commits]

## Testing

- [ ] `bun run check` passes (0 new errors)
- [ ] `bun run test:unit` passes
- [ ] Manual testing completed
- [ ] Dark mode verified (if UI changes)

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Enhancement
- [ ] Refactoring
- [ ] Documentation

---
**PR Created by:** Claude Code Agent
```

## Reviewer Selection Logic

1. **Default Reviewers**: If `.reviewers.json` exists, parse it to add reviewers based on:
   - File paths changed
   - File extensions
   - Global reviewers
2. Add reviewers via: `gh pr edit {PR_NUMBER} --add-reviewer {REVIEWER}`

## Pre-PR Checklist

Before creating the PR, verify:
1. `bun run check` — 0 new TypeScript errors (pre-existing warnings OK)
2. No credentials or secrets in changed files
3. CSS follows component namespace convention (`nf-*`, `.col-*`, etc.)
4. Svelte 5 runes used correctly (no `$derived` in getters)
5. `.svelte.ts` extension used for rune-containing non-component files

## Merge Conflict Detection

**IMPORTANT**: This process NEVER pushes to target/main. It only:
- Pushes YOUR working branch to origin
- Creates a Pull Request from your working branch TO the specified target branch
- Checks for conflicts locally before creating the PR

### Steps to Check for Conflicts:

1. **Fetch latest changes from remote:**
   ```bash
   git fetch origin
   ```

2. **Check if target branch has new commits:**
   ```bash
   git log HEAD..origin/{target-branch} --oneline
   ```

3. **If target branch has new commits, test merge locally:**
   ```bash
   git merge origin/{target-branch}
   ```

4. **Handle merge results:**
   - **No conflicts**: Continue with PR creation
   - **Conflicts detected**: Prompt user for resolution

## Usage

This prompt will:
1. Parse the optional target branch argument (defaults to `main`)
2. Validate branch state and run type checks
3. Check for merge conflicts WITHOUT pushing to target
4. Push ONLY your working branch to origin
5. Generate appropriate PR title and description from commits
6. Create PR via `gh pr create`
7. Add reviewers if `.reviewers.json` exists
8. Return the PR URL

## Important Notes

- **NEVER pushes directly to target branches (main, master)**
- **Only pushes your working branch to origin**
- **Pull Requests are always created for merging into protected branches**
- **The actual merge only happens when PR is approved and merged via GitHub UI**
