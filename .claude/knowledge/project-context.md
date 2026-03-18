# Project Context - NexusFlow

Project-specific configuration, URLs, and references for agent work.

## Project Identification

- **Project Name:** NexusFlow
- **Description:** Universal Multi-Cloud Control — a superior Postman alternative for API testing in containerized multi-cloud workloads
- **Stack:** SvelteKit 5 · Svelte 5 Runes · TypeScript · Monaco Editor · @xyflow/svelte · GSAP · ELK.js

## Repository

- **Platform:** GitHub
- **CLI:** `gh` (GitHub CLI)
- **Default Branch:** `main`
- **Remote:** `origin`

## Development Environment

- **Runtime:** Bun 1.x
- **Dev Server:** `bun run dev` → http://localhost:3200
- **Build:** `bun run build` (adapter-node)
- **Type Check:** `bun run check` — target: 0 errors, ~42 pre-existing warnings
- **Unit Tests:** `bun run test:unit` (Vitest)
- **E2E Tests:** `bun run test:e2e` (Playwright, base URL: localhost:3200)

## Branch Strategy

- **Main branch:** `main` (protected, production-ready)
- **Feature branches:** `feature/{description}`, `fix/{description}`, `refactor/{description}`
- **No develop branch** — features branch from and merge to `main`

## Codebase Paths

- **Source code:** `src/`
- **Components:** `src/lib/components/`
- **Stores:** `src/lib/stores/`
- **Utils:** `src/lib/utils/`
- **Routes/API:** `src/routes/`
- **Tests:** `tests/` (E2E), `src/lib/**/*.test.ts` (unit)
- **Styles:** `src/lib/styles/`
- **Workers:** `src/lib/workers/`

## Available Agent Roles

| Agent | Specialization |
|-------|---------------|
| `frontend-engineer` | SvelteKit 5, Svelte 5 runes, Monaco, @xyflow/svelte |
| `senior-qa-engineer` | Vitest + Playwright testing for NexusFlow |
| `distinguished-engineer` | Architecture, technical strategy, system design |
| `ux-designer` | UX/UI design, accessibility, design systems |
| `code-reviewer` | Code quality, security, best practices |
| `typescript-pro` | Advanced TypeScript patterns, type safety |
| `cloud-architect` | Multi-cloud infrastructure, AWS/Azure/GCP |
| `deployment-engineer` | CI/CD pipelines, deployment automation |
| `docker-expert` | Container images, orchestration |

## Key Design Principles

1. **No Git integration in the app** — NexusFlow does not sync with Git
2. **Local-first storage** — localStorage → IndexedDB → optional cloud
3. **UX is king** — simplicity over features, one CTA per view
4. **Text labels on navigation** — icon+text, never icon-only
5. **Zero extra dependencies** unless truly necessary
6. **0 new TypeScript errors** after every change

## Work Tracking

- **No external issue tracker** (no JIRA, Linear, etc.)
- Work is tracked via Claude Code tasks and local planning docs
- Planning documents go in `docs/planning/` when needed

---
*Run `/setup-project` to customize this file for your specific environment.*
