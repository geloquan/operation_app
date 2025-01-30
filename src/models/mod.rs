use std::sync::{Arc, RwLock};

use operation::OperationModel;

pub(crate) trait ConfigExecutor {
    fn execute(&mut self, config: Config);
}
pub(crate) struct Config {
    ascending: Option<bool>,
    search: Option<String>
}
impl Default for Config {
    fn default() -> Self {
        Self {
            ascending: None,
            search: None
        }
    }
}
pub mod operation;
mod tool {
    enum Status {
        Ready,
        Unavailable,
        OnMaintenance
    }
    pub(crate) struct Tool {
        id: i32,
        tool_name: String,
        tool_status: Status
    }

    pub(crate) type ToolModel = Vec<Tool>;
}
enum Table {
    Operation(Config),
    Tool(Config)
}

pub(crate) struct Model;

impl Model {
    //pub fn get<T> (&self, table: Table, stream: Arc<std::sync::RwLock<StreamDatabase>>) -> Result<TableReturn, error::Error> {
    //    match table {
    //        Table::Operation(config) => Ok(TableReturn::Operation(self.operation(config, stream))),
    //        Table::Tool(config) => Ok(TableReturn::Tools(self.tool(config, stream))),
    //    }
    //}

    pub fn get_operation(config: Config, stream: &Arc<std::sync::RwLock<StreamDatabase>>) -> operation::OperationModel {
        let binding = stream.write().unwrap();
        let mut operation = binding.operation.write().unwrap();
        operation.execute_config(config)
        //stream.operation.execute1(config);
        //stream.operation
    }

    fn tool(&self, config: Config, stream: Arc<std::sync::RwLock<StreamDatabase>>) -> tool::ToolModel {
        todo!()
    }
}

mod error {
    pub(crate) enum Error {
        NotRowFound,
    }
}

enum TableReturn {
    Operation(operation::OperationModel),
    Tools(tool::ToolModel)
}
pub(crate) struct StreamDatabase {
    operation: Arc<RwLock<operation::OperationModel>>,
}
impl StreamDatabase {
    pub fn init(operation: operation::OperationModel) -> Self {
        Self {
            operation: Arc::new(RwLock::new(operation))
        }
    }
    pub fn get_operation(&self) -> operation::OperationModel {
        self.operation.read().unwrap().clone()
    }
}