
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