use super::join::structure::{OperationSelect, PreOperativeToolReady};


#[derive(Debug, Clone)]
pub enum WindowTable {
    OperationSelect(Option<Vec<OperationSelect>>),
    PreOperativeToolReady(Option<Vec<PreOperativeToolReady>>)
}