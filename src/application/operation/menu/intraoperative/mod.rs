use serde::{Deserialize, Serialize};


#[derive(Clone, Default)]
pub struct Menu {
    pub selected_menu: Option<MenuOptions>,
    pub selected_action: Option<Action>,
}

#[derive(PartialEq, Clone)]
pub enum MenuOptions {
    RoomStatus
}

#[derive(PartialEq, Clone)]
pub enum Action {
    ToolRequest,
    StaffRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    
}