use std::sync::{Arc, RwLock};

use exchange_format::SessionToken;
use operation::OperationModel;

use crate::views::{State, View};

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

pub mod exchange_format;

impl Model {

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
    Tools(tool::ToolModel),

}
pub(crate) struct StreamDatabase {
    operation: Arc<RwLock<operation::OperationModel>>,
    app_state: Arc<RwLock<View>>,
    session_token: Arc<RwLock<SessionToken>>
}
impl StreamDatabase {
    pub fn init(operation: operation::OperationModel) -> Self {
        Self {
            operation: Arc::new(RwLock::new(operation)),
            app_state: Arc::new(RwLock::new(View::default())),
            session_token: Arc::new(RwLock::new(SessionToken(None)))
        }
    }
    pub fn get_operation(&self) -> operation::OperationModel {
        self.operation.read().unwrap().clone()
    }
    pub fn get_app_state(&self) -> View {
        self.app_state.read().unwrap().clone()
    }
    pub fn new_app_state(&mut self, state: View) {
        let mut app = self.app_state.write().unwrap();
        *app = state;
    }
    pub fn new_session_token(&mut self, new_session_token: &SessionToken) {
        let mut session_token_lock = self.session_token.write().unwrap();
        *session_token_lock = new_session_token.clone();
    }
}