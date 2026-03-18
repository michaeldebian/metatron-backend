-- Metatron Backend — Initial Schema
-- Auth, RBAC, Resources, MCP Connections, Audit Log

-- ── Organizations ────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ── Users ────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id),
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    avatar_url TEXT,
    password_hash TEXT,
    identity_provider TEXT NOT NULL DEFAULT 'local',
    is_org_owner BOOLEAN NOT NULL DEFAULT false,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(org_id, email)
);

-- ── Teams ────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id),
    name TEXT NOT NULL,
    description TEXT,
    color TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(org_id, name)
);

CREATE TABLE IF NOT EXISTS team_members (
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'viewer',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (team_id, user_id)
);

-- ── Permission Sets ──────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS permission_sets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID REFERENCES organizations(id),
    name TEXT NOT NULL,
    description TEXT,
    is_system BOOLEAN NOT NULL DEFAULT false,
    grants JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Unique constraint for system permission sets (org_id IS NULL)
CREATE UNIQUE INDEX IF NOT EXISTS idx_permission_sets_system_name
    ON permission_sets (name) WHERE org_id IS NULL AND is_system = true;

CREATE TABLE IF NOT EXISTS team_permission_sets (
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    permission_set_id UUID NOT NULL REFERENCES permission_sets(id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, permission_set_id)
);

-- ── User Permission Overrides ────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS user_permission_overrides (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    granted BOOLEAN NOT NULL,
    PRIMARY KEY (user_id, module, action)
);

-- ── Cloud Credentials ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS cloud_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id),
    provider TEXT NOT NULL,
    profile_name TEXT NOT NULL,
    encrypted_config BYTEA NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    status TEXT NOT NULL DEFAULT 'unchecked',
    last_verified_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS credential_team_grants (
    credential_id UUID NOT NULL REFERENCES cloud_credentials(id) ON DELETE CASCADE,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (credential_id, team_id)
);

-- ── Resources (pipelines, collections, notebooks, dashboards) ────────────────
CREATE TABLE IF NOT EXISTS resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id),
    resource_type TEXT NOT NULL,
    name TEXT NOT NULL,
    owner_user_id UUID REFERENCES users(id),
    owner_team_id UUID REFERENCES teams(id),
    visibility TEXT NOT NULL DEFAULT 'private',
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_resources_org_type ON resources(org_id, resource_type);
CREATE INDEX IF NOT EXISTS idx_resources_owner_user ON resources(owner_user_id);
CREATE INDEX IF NOT EXISTS idx_resources_owner_team ON resources(owner_team_id);

-- ── MCP Connections (end-user MCP servers) ───────────────────────────────────
CREATE TABLE IF NOT EXISTS mcp_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    org_id UUID NOT NULL REFERENCES organizations(id),
    name TEXT NOT NULL,
    transport TEXT NOT NULL,
    endpoint_url TEXT,
    command TEXT,
    args JSONB,
    env_vars JSONB,
    status TEXT NOT NULL DEFAULT 'disconnected',
    last_connected_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_mcp_connections_user ON mcp_connections(user_id);

-- ── Audit Log ────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL,
    user_id UUID NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT,
    resource_id UUID,
    metadata JSONB,
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_audit_org_time ON audit_log(org_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_user ON audit_log(user_id, created_at DESC);
