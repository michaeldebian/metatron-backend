---
description: Configure project-specific settings for NexusFlow
---

# Setup Project

Interactive questionnaire to configure project-specific settings for NexusFlow development.

## Instructions

When this command is run, guide the user through configuring their development environment by asking questions and updating necessary files.

## Step 1: Gather Project Information

Ask the user the following questions:

### Repository Information
1. **Repository URL**: What is the GitHub repository URL? (e.g., "https://github.com/org/nexusflow")
2. **Default Branch**: What is the default branch? (default: `main`)

### Development Environment
3. **Dev Server Port**: What port for the dev server? (default: `3200`)
4. **Node Runtime**: Bun or Node? (default: `Bun`)

### Optional: Confluence
5. **Use Confluence?**: Will this project use Confluence for documentation?
   - Options: Yes, No
6. (If Yes) **Confluence Space Key**: What is the Confluence space key?
7. (If Yes) **Confluence Base URL**: What is the Confluence server URL?

### Team
8. **Project Lead**: Who is the project lead/owner? (name)

## Step 2: Update Configuration Files

After gathering all information, update the following files:

### 1. `.claude/knowledge/project-context.md`
Fill in all project-specific values.

### 2. `.reviewers.json` (if needed)
```json
{
  "default_reviewers": ["{DEFAULT_REVIEWER}"]
}
```

## Step 3: Verify Development Environment

```bash
# Verify dev server starts
bun run dev

# Verify type checking passes
bun run check

# Verify tests pass
bun run test:unit
```

## Step 4: Summary

After completing setup, display a summary:

```
Project Setup Complete!
========================

Project: NexusFlow
Repository: {REPOSITORY_URL}
Dev Server: http://localhost:{PORT}
Runtime: {RUNTIME}

Files Updated:
- .claude/knowledge/project-context.md
- .reviewers.json (if applicable)

Next Steps:
1. Review the updated project-context.md for accuracy
2. Run `bun run dev` to start development
3. Run `bun run check` to verify 0 new errors

Available Commands:
- /summon {agent}     - Activate an agent persona
- /create-pr          - Create a pull request
- /review-pr {NUMBER} - Review a pull request
- /review-security    - Security review of changes
- /clean              - Clean up worktrees and dev servers

Agent Team:
- frontend-engineer   - SvelteKit 5 / Svelte 5 runes specialist
- senior-qa-engineer  - Vitest + Playwright testing
- distinguished-engineer - Architecture & technical strategy
- ux-designer         - UX/UI design and accessibility
- code-reviewer       - Code quality and security
- typescript-pro      - Advanced TypeScript patterns
```

## Important Notes

- Run this command ONCE when setting up the project
- After setup, the configuration is ready for development
- This project does NOT use JIRA, Bitbucket, or external issue trackers
- Work tracking is done locally via Claude Code tasks and planning docs
