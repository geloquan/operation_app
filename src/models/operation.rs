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

impl Default for Operation {
    fn default() -> Self {
        Self {
            id: 0,
            operation_name: "".to_string(),
            operation_status: Status::Preoperation
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct OperationModel(Vec<Operation>);

impl OperationModel {
    pub fn new(s: Vec<Operation>) -> Self {
        Self(s)
    } 
    pub fn execute_config(&mut self, config: super::Config) -> OperationModel {
        &self.execute(config);
        self.clone()
    }
}

impl ConfigExecutor for OperationModel {
    fn execute(&mut self, config: super::Config) {
        if let Some(search_term) = config.search {
            self.0.retain(|op| {
                op.operation_name
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            });
        }
        if let Some(ascending) = config.ascending {
            self.0.sort_by(|a, b| {
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
