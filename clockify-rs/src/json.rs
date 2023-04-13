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

    pub name: String,
    pub billable: bool,

    pub public: bool,
}

/// The high level header information for the project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub id: String,

    #[serde(rename = "clientId")]
    pub client_id: String,
    pub name: String,
    pub billable: bool,

    pub public: bool,

    pub color: String,

    pub note: String,
}

/// A single task within a project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub status: String,
}

/// A single task within a project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub status: String,
    pub email: String,
}

/// A single task within a project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub status: String,
}
