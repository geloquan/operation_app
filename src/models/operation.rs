use serde_json::error;

use super::ConfigExecutor;

#[derive(Clone, Debug)]
enum Status {
    Preoperation,
    Intraoperation,
    Postoperation
}

#[derive(Clone, Debug)]
pub(crate) struct Operation {
    id: i32,
    operation_name: String,
    operation_status: Status
}

#[derive(Clone, Debug)]
pub(crate) struct OperationModel(pub Option<Vec<Operation>>);

impl OperationModel {
    pub fn new(s: Option<Vec<Operation>>) -> Self {
        Self(s)
    } 
    pub fn execute_config(&mut self, config: super::Config) -> OperationModel {
        &self.execute(config);
        self.clone()
    }
}

impl ConfigExecutor for OperationModel {
    fn execute(&mut self, config: super::Config) {
        if let Some(ref mut operation_model) = self.0 {
            if let Some(ref search_term) = config.search {
                let search_term = search_term.to_lowercase();
                operation_model.retain(|op| op.operation_name.to_lowercase().contains(&search_term));
            }
            
            if let Some(ascending) = config.ascending {
                operation_model.sort_by(|a, b| {
                    let ord = a.operation_name.cmp(&b.operation_name);
                    if ascending {
                        ord
                    } else {
                        ord.reverse()
                    }
                });
            }
        }
    }
}
