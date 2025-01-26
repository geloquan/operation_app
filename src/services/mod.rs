mod server;
mod app;

trait Init {
    fn init(&self) -> Result<(), &'static str>;
}

pub(crate) struct Service {
    server: server::Comms,
    app: app::Comms
}

impl Init for Service {
    fn init(&self) -> Result<(), &'static str> {
        self.server = server::Comms::init(&self).map_err("server err");
        self.app = app::Comms::init(&self).map_err("app err");

        Ok(())
    }
}