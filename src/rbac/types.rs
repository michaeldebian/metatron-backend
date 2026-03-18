use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Module IDs matching the frontend nav-config.ts ModuleId type.
pub const MODULES: &[&str] = &[
    "dashboard",
    "my-files",
    "network",
    "terminal",
    "notebook",
    "pipeline",
    "analytics",
    "alerts",
    "clusters",
    "finops",
    "registry",
    "vault",
    "identity",
    "deployer",
    "wizard",
    "servicenow",
    "settings",
    "logs",
];

/// Action IDs for permission grants.
pub const ACTIONS: &[&str] = &["view", "create", "edit", "delete", "execute", "export", "admin"];

/// Fully resolved permissions for a user, computed from team permission sets + overrides.
#[derive(Debug, Clone, Default)]
pub struct ResolvedPermissions {
    /// module → set of allowed actions
    pub grants: HashMap<String, HashSet<String>>,
    /// Team IDs the user belongs to (for resource filtering)
    pub team_ids: Vec<Uuid>,
    /// Credential IDs accessible through team grants
    pub credential_ids: Vec<Uuid>,
}

impl ResolvedPermissions {
    pub fn can(&self, module: &str, action: &str) -> bool {
        self.grants
            .get(module)
            .map(|actions| actions.contains(action))
            .unwrap_or(false)
    }

    pub fn visible_modules(&self) -> Vec<String> {
        self.grants
            .iter()
            .filter(|(_, actions)| actions.contains("view"))
            .map(|(module, _)| module.clone())
            .collect()
    }

    /// Convert to the format stored in JWT claims.
    pub fn to_claims_map(&self) -> HashMap<String, Vec<String>> {
        self.grants
            .iter()
            .map(|(module, actions)| {
                (module.clone(), actions.iter().cloned().collect())
            })
            .collect()
    }
}

/// System-defined permission sets that map to the existing developer1 personas.
pub struct SystemPermissionSet {
    pub name: &'static str,
    pub description: &'static str,
    pub grants: &'static [(&'static str, &'static [&'static str])],
}

/// Standard actions for most modules.
const VEE: &[&str] = &["view", "create", "edit", "execute"];
const VIEW_ONLY: &[&str] = &["view"];

pub const SYSTEM_PERMISSION_SETS: &[SystemPermissionSet] = &[
    SystemPermissionSet {
        name: "developer",
        description: "Software developer — API testing, pipelines, notebooks",
        grants: &[
            ("dashboard", VIEW_ONLY),
            ("my-files", VEE),
            ("network", VEE),
            ("terminal", VEE),
            ("notebook", VEE),
            ("pipeline", VEE),
            ("logs", VIEW_ONLY),
            ("analytics", VIEW_ONLY),
        ],
    },
    SystemPermissionSet {
        name: "ml",
        description: "ML engineer — notebooks, pipelines, analytics",
        grants: &[
            ("dashboard", VIEW_ONLY),
            ("my-files", VEE),
            ("network", VEE),
            ("terminal", VEE),
            ("notebook", VEE),
            ("pipeline", VEE),
            ("logs", VIEW_ONLY),
            ("analytics", VEE),
        ],
    },
    SystemPermissionSet {
        name: "devops",
        description: "DevOps engineer — clusters, pipelines, vault, alerts",
        grants: &[
            ("dashboard", VIEW_ONLY),
            ("network", VEE),
            ("terminal", VEE),
            ("pipeline", VEE),
            ("clusters", VEE),
            ("registry", VEE),
            ("vault", VEE),
            ("logs", VIEW_ONLY),
            ("alerts", VEE),
            ("deployer", VEE),
        ],
    },
    SystemPermissionSet {
        name: "platform",
        description: "Platform engineer — full infrastructure management",
        grants: &[
            ("dashboard", VIEW_ONLY),
            ("pipeline", VEE),
            ("clusters", VEE),
            ("registry", VEE),
            ("vault", VEE),
            ("identity", VEE),
            ("logs", VIEW_ONLY),
            ("settings", &["view", "create", "edit", "delete", "admin"]),
            ("finops", VEE),
            ("alerts", VEE),
            ("deployer", VEE),
        ],
    },
    SystemPermissionSet {
        name: "finops",
        description: "FinOps analyst — cost dashboards and analytics",
        grants: &[
            ("dashboard", VIEW_ONLY),
            ("clusters", VIEW_ONLY),
            ("finops", VIEW_ONLY),
            ("logs", VIEW_ONLY),
            ("analytics", VIEW_ONLY),
        ],
    },
];
