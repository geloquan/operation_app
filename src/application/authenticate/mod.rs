use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct StaffCredential {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub session_token: String
}