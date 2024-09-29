use super::window::WindowTable;


#[derive(Debug, Clone)]
pub struct TableTree {
    pub data: WindowTable,
    pub child: Option<Box<TableTree>>
}