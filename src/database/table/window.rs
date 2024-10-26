use super::join::structure::{OperationSelect, OperationStaffProperty, PreOperativeToolReady};


#[derive(Debug, Clone)]
pub enum WindowTable {
    OperationSelect(Option<Vec<OperationSelect>>),
    PreOperativeToolReady(Option<Vec<PreOperativeToolReady>>),
    PreoperativeStaffList(Option<Vec<OperationStaffProperty>>)
}