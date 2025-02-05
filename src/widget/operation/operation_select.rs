use crate::models::operation::Operation;

#[derive(Default)]
pub(crate) struct OperationSelect {
    pub search: String,
    pub operation: Option<Operation>
}