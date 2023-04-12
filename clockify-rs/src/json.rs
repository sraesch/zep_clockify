use serde::{Deserialize, Serialize};

/// The high level header information for the workspace
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceHeader {
    pub id: String,
    pub name: String,
}

/// The high level header information for the project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectHeader {
    pub id: String,

    #[serde(rename = "clientId")]
    pub client_id: String,
    pub name: String,
    pub billable: bool,

    pub public: bool,
}
