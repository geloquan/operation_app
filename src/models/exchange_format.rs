use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    CheckUser,
}
pub struct CheckUserReturn {
    pub valid: bool,
    pub full_name: String,
    pub session_token: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerExchangeFormat {
    pub request: bool,
    pub method: Method,
    pub metadata: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct SessionToken(pub Option<String>);