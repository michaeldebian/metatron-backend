---
description: Clean up worktrees and optionally stop dev servers
argument-hint: "[--dev] [--force] [BRANCH]"
---

# Clean Worktrees and Dev Servers

This command cleans up git worktrees with optional control over development server cleanup.

## Usage

- `/clean` - List active worktrees and their status
- `/clean --dev` - Kill all dev servers on common ports
- `/clean --force` - Force delete ALL worktrees without confirmation
- `/clean BRANCH` - Clean up a specific worktree by branch name
- `/clean --force --dev` - Force delete ALL worktrees AND kill all dev servers

### Parameters

- `--dev` - Kill all running development servers on common ports (3200, 3000, 5000, 5173, 8080)
- `--force` - Force delete all worktrees without checking for uncommitted changes
- `BRANCH` - Specific branch name to clean. If omitted, cleans ALL worktrees

## Steps to Execute

### Parameter Detection

1. **Parse parameters from arguments**
   - Check if `--dev` flag is present -> set `KILL_DEV_SERVERS=true`
   - Check if `--force` flag is present -> set `FORCE_DELETE=true`
   - Extract remaining argument as `BRANCH` (if provided)

### If No BRANCH Provided (Clean ALL)

1. **Kill all running development servers (ONLY if --dev flag provided)**
   - Find and kill any processes running on ports: 3200, 3000, 5000, 5173, 8080
   - Check for any bun/node processes related to the project
   - Skip this step entirely if `--dev` not specified

2. **List and remove all worktrees**
   - Run `git worktree list` to see all active worktrees
   - For each worktree (except main/default):
     - If `--force` flag: Skip uncommitted changes check and use `git worktree remove --force`
     - If no `--force` flag: Check for uncommitted changes with `git status`
       - Warn user if uncommitted changes exist
       - Run `git worktree remove <worktree-path>` or confirm before force removing

3. **Update to latest main**
   - Ensure you're in the project root directory
   - Stash any uncommitted changes: `git stash`
   - Checkout main: `git checkout main`
   - Pull latest: `git pull origin main`

4. **Verify clean state**
   - Run `git status` to confirm clean working tree
   - Run `git worktree list` to confirm only default remains

### If BRANCH Provided (Clean Single Worktree)

1. **Find the specified worktree**
   - Search for worktree matching the BRANCH parameter

2. **Stop related development server (ONLY if --dev flag provided)**
   - Check if any processes are running in that worktree directory
   - Kill only processes related to that specific worktree

3. **Remove the specific worktree**
   - If `--force` flag: Skip uncommitted changes check
   - If no `--force` flag: Check for uncommitted changes and warn
   - Remove worktree: `git worktree remove <worktree-path>`

## Safety Checks

- **Before removing worktrees**: Warn if any have uncommitted changes
- **Before killing processes**: Show which processes will be killed
- **After cleanup**: Confirm everything was successful

## Example Output

```
Cleanup Process Started
================================

1. Stopping dev servers (--dev flag)...
   Killed bun process on port 3200 (PID: 12345)

2. Removing all worktrees...
   Removed: worktrees/feature-auth-panel
   Removed: worktrees/fix-proxy-timeout

3. Updating to latest main...
   Checked out main
   Pulled latest from origin/main

4. Verification...
   Working tree is clean
   No active worktrees except default

Cleanup complete! You're on main with the latest code.
```
