# API Reference

Quick reference for external tool integrations used by agents.

---

## GitHub CLI (`gh`)

All repository operations use the GitHub CLI. Verify authentication with:

```bash
gh auth status
```

### Pull Requests

```bash
# Create PR
gh pr create --base main --title "Title" --body "Body"

# View PR details
gh pr view {PR_NUMBER} --json title,body,files,additions,deletions,baseRefName,headRefName

# Get PR diff
gh pr diff {PR_NUMBER}

# Comment on PR
gh pr comment {PR_NUMBER} --body "Comment text"

# Add reviewers
gh pr edit {PR_NUMBER} --add-reviewer {USERNAME}

# List open PRs
gh pr list

# Check PR status
gh pr checks {PR_NUMBER}
```

### Issues (GitHub Issues)

```bash
# Create issue
gh issue create --title "Title" --body "Body"

# List issues
gh issue list

# View issue
gh issue view {ISSUE_NUMBER}

# Close issue
gh issue close {ISSUE_NUMBER}
```

### Repository

```bash
# Clone
gh repo clone {OWNER}/{REPO}

# View repo info
gh repo view

# List releases
gh release list
```

---

## NexusFlow Internal APIs

These are the server-side API routes in the NexusFlow application itself:

### HTTP Proxy — `POST /api/proxy`

```typescript
// Request:
{ method: string, url: string, headers: Record<string,string>,
  body: string | null, skipTlsVerify?: boolean }

// Response:
{ status: number, statusText: string, headers: Record<string,string>,
  body: string, timing: number, size: number, error?: string,
  truncated?: boolean }
```

- 30s timeout via AbortController
- 10 MB response cap (sets `truncated: true`)
- URL must be `http://` or `https://`

### gRPC Proxy — `POST /api/grpc-proxy`

Unary RPCs via `@grpc/grpc-js`. No streaming support yet.

### Database Proxy — `POST /api/db-proxy`

Server-side database query execution.

---

## Confluence REST API (Optional)

If Confluence is configured:

```bash
# Get page by title
bash -c 'curl -s -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" \
     -G --data-urlencode "title={PAGE_TITLE}" \
     --data-urlencode "spaceKey={SPACE_KEY}" \
     --data-urlencode "expand=body.storage" \
     "$CONFLUENCE_BASE_URL/rest/api/content"'

# Update page
bash -c 'curl -s -X PUT \
     -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" \
     -H "Content-Type: application/json" \
     -d "{JSON_PAYLOAD}" \
     "$CONFLUENCE_BASE_URL/rest/api/content/{PAGE_ID}"'
```

---

## Agent Attribution

**All agents must identify themselves when creating external artifacts.**

Format:
```
**[Agent Name]** - [Action/Status]

[Details]
```

See `.claude/includes/attribution-reminder.md` for complete attribution policy.
