
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffAuthGrant {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub session_token: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OperationToolOnSiteToggle {
    pub operation_id: i32,
    pub tool_id: i32,
    pub on_site_value: bool,
}