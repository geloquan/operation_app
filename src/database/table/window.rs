use super::join::structure::OperationSelect;


#[derive(Debug, Clone)]
pub enum WindowTable {
    OperationSelect(Option<Vec<OperationSelect>>),
}