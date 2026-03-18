---
description: Update or create Confluence documentation pages with proper agent attribution
argument-hint: PAGE_TITLE
---

# Update Confluence

Update or create Confluence documentation pages with proper agent attribution.

## Command Syntax

```
/update-confluence {PAGE_TITLE}
```

## Example Usage

- `/update-confluence Team Structure` - Update the Team Structure page
- `/update-confluence Implementation Plan` - Update implementation plan
- `/update-confluence API Documentation` - Create/update API docs
- `/update-confluence Technical Specification` - Create/update tech spec

## Agent Attribution Requirements

**See:** `.claude/includes/attribution-reminder.md` for agent attribution requirements.

**Attribution Methods:**

### 1. Page Footer (for major updates or new pages)

Add to the bottom of the page content:

```markdown
---
*Documentation prepared by [AGENT_NAME] ([ROLE]) - [DATE]*
```

### 2. Page Comment (for minor updates)

Post a comment on the page after updating:

```markdown
**[AGENT_NAME]** - Updated [description of what changed].

Changes:
- [Change 1]
- [Change 2]
```

### 3. Version Notes (optional, for transparency)

When making the API call, include descriptive version message:
```json
{
  "version": {
    "number": 2,
    "minorEdit": false,
    "message": "Updated by [AGENT_NAME]: [brief description]"
  }
}
```

## Confluence API Configuration

**Base URL**: Use `$CONFLUENCE_BASE_URL` environment variable
**Space Key**: Configure in `.confluence/project.json` or use project-specific value
**Authentication**: Use `CONFLUENCE_API_TOKEN` with Bearer token
**Content-Type**: `application/json`

**See:** `.claude/includes/authentication-reminder.md` for authentication requirements.

```bash
bash -c 'curl -s -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" ...'
```

## Configuration File (Optional)

Create `.confluence/project.json` to store project-specific settings:

```json
{
  "space_key": "YOUR_SPACE_KEY",
  "parent_page_id": "123456789",
  "base_url": "https://docs.example.com"
}
```

## Workflow Steps

### 1. Read Configuration

```bash
SPACE_KEY=$(jq -r '.space_key' .confluence/project.json 2>/dev/null || echo "")
PARENT_PAGE_ID=$(jq -r '.parent_page_id' .confluence/project.json 2>/dev/null || echo "")
```

### 2. Search for Existing Page

```bash
bash -c 'curl -s -X GET "$CONFLUENCE_BASE_URL/rest/api/content?spaceKey={SPACE_KEY}&title={PAGE_TITLE}&expand=body.storage,version" \
  -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" \
  -H "Content-Type: application/json"'
```

### 3. Determine Action

- **Page exists** -> Update existing page (PUT request)
- **Page not found** -> Create new page (POST request)

### 4a. Update Existing Page

```bash
bash -c 'curl -s -X PUT "$CONFLUENCE_BASE_URL/rest/api/content/{PAGE_ID}" \
  -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '"'"'{
    "id": "{PAGE_ID}",
    "type": "page",
    "title": "{PAGE_TITLE}",
    "version": {
      "number": {CURRENT_VERSION + 1},
      "minorEdit": false,
      "message": "Updated by [AGENT_NAME]: [brief description]"
    },
    "body": {
      "storage": {
        "value": "<h1>Page Content</h1><p>Content here...</p><hr /><p><em>Documentation prepared by [AGENT_NAME] ([ROLE]) - [DATE]</em></p>",
        "representation": "storage"
      }
    }
  }'"'"''
```

### 4b. Create New Page

```bash
bash -c 'curl -s -X POST "$CONFLUENCE_BASE_URL/rest/api/content" \
  -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "type": "page",
    "title": "{PAGE_TITLE}",
    "space": {
      "key": "{SPACE_KEY}"
    },
    "ancestors": [
      {"id": "{PARENT_PAGE_ID}"}
    ],
    "body": {
      "storage": {
        "value": "<h1>Page Content</h1><p>Content here...</p><hr /><p><em>Documentation prepared by [AGENT_NAME] ([ROLE]) - [DATE]</em></p>",
        "representation": "storage"
      }
    }
  }'
```

### 5. Post Comment (for updates)

For existing pages being updated, also post a comment:

```bash
bash -c 'curl -X POST "$CONFLUENCE_BASE_URL/rest/api/content/{PAGE_ID}/child/comment" \
  -H "Authorization: Bearer $CONFLUENCE_API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "body": {
      "storage": {
        "value": "<p><strong>[AGENT_NAME]</strong> - Updated [description].</p><p>Changes:</p><ul><li>[Change 1]</li><li>[Change 2]</li></ul>",
        "representation": "storage"
      }
    }
  }'
```

## Content Conversion

Confluence uses XHTML storage format. Common conversions:

| Markdown | Confluence XHTML |
|----------|------------------|
| `# Header` | `<h1>Header</h1>` |
| `## Header` | `<h2>Header</h2>` |
| `**bold**` | `<strong>bold</strong>` |
| `*italic*` | `<em>italic</em>` |
| `- list item` | `<ul><li>list item</li></ul>` |
| `1. list item` | `<ol><li>list item</li></ol>` |
| `` `code` `` | `<code>code</code>` |
| Horizontal rule | `<hr />` |

**Tables:**
```html
<table>
  <thead>
    <tr><th>Header 1</th><th>Header 2</th></tr>
  </thead>
  <tbody>
    <tr><td>Cell 1</td><td>Cell 2</td></tr>
  </tbody>
</table>
```

## Use Cases

### 1. Update Team Documentation

```
/update-confluence Team Structure
```
- Reads local `docs/team-structure.md` (if exists)
- Converts markdown to XHTML
- Updates Confluence page
- Adds footer attribution
- Posts comment describing changes

### 2. Publish Planning Documents

```
/update-confluence Implementation Plan
```
- Reads local planning document
- Creates new page if doesn't exist
- Adds attribution footer
- Sets proper parent page hierarchy

### 3. Create Technical Specifications

```
/update-confluence Technical Specification - Phase 2
```
- Creates new page under configured parent
- Generates spec from code analysis
- Includes agent attribution

## API Endpoints

- **Search Content**: `GET /rest/api/content?spaceKey={KEY}&title={TITLE}&expand=body.storage,version`
- **Get Page**: `GET /rest/api/content/{PAGE_ID}?expand=body.storage,version`
- **Create Page**: `POST /rest/api/content`
- **Update Page**: `PUT /rest/api/content/{PAGE_ID}`
- **Add Comment**: `POST /rest/api/content/{PAGE_ID}/child/comment`

## Error Handling

- **Page not found** -> Offer to create new page
- **Version conflict** -> Fetch latest version and retry
- **Invalid XHTML** -> Validate content before posting
- **Permission denied** -> Verify CONFLUENCE_API_TOKEN is set
- **Space not found** -> Verify space key is correct

## Safety Features

- Always fetch current version before updating
- Preserve existing content when making targeted updates
- Validate XHTML syntax before posting
- Add clear attribution for all changes
- Maintain page hierarchy and parent relationships

## Success Output

```
Confluence Page Updated Successfully

Page: Team Structure
URL: {CONFLUENCE_BASE_URL}/display/{SPACE_KEY}/Team+Structure
Version: 2 -> 3

Changes:
- Updated section on development workflow
- Added new team member information
- Updated project timeline

Attribution: [AGENT_NAME] ([ROLE])
Comment posted: Yes
```

## Environment Variables

| Variable | Purpose | Required |
|----------|---------|----------|
| `CONFLUENCE_API_TOKEN` | Authentication token | Yes |
| `CONFLUENCE_BASE_URL` | Confluence server URL | Yes |

## Benefits

- Centralizes documentation updates
- Maintains consistent attribution
- Reduces manual copy-paste between markdown and Confluence
- Preserves documentation provenance
- Enables automated documentation publishing

## Related Commands

- `/create-pr` - For changes that need technical documentation
- `/review-pr` - For reviewing changes before merge

## Usage Notes

**IMPORTANT:**
- Always add agent attribution via footer or comment
- For major updates, use both footer attribution AND a comment
- Keep local markdown files in `docs/` as source of truth when applicable
- Confluence should mirror the local documentation
- When in doubt, add more attribution rather than less
