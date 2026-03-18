# Authentication Reminder

## GitHub CLI (`gh`)

NexusFlow uses GitHub for repository management. Use the GitHub CLI (`gh`) for all repository operations.

### Verify Authentication

```bash
gh auth status
```

### Common Operations

```bash
# Create PR
gh pr create --base main --title "Title" --body "Body"

# View PR
gh pr view {PR_NUMBER}

# Get PR diff
gh pr diff {PR_NUMBER}

# Comment on PR
gh pr comment {PR_NUMBER} --body "Comment text"

# Add reviewers
gh pr edit {PR_NUMBER} --add-reviewer {USERNAME}
```

## Confluence (Optional)

If Confluence is configured, API calls use Bearer token authentication:

```bash
bash -c 'curl -s -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" ...'
```

### Required Environment Variables (if using Confluence)

| Variable | Purpose | Required |
|----------|---------|----------|
| `CONFLUENCE_API_TOKEN` | Confluence authentication | Only if using Confluence |
| `CONFLUENCE_BASE_URL` | Confluence server URL | Only if using Confluence |

## Troubleshooting

If `gh` commands fail:
1. Run `gh auth status` to verify authentication
2. Run `gh auth login` if not authenticated
3. Ensure the repository remote is set correctly: `git remote -v`
