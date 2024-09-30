
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffAuthGrant {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub session_token: Option<String>,
}
