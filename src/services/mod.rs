mod server;
mod app;

trait Init {
    fn init() -> Result<Self, &'static str> where Self: Sized;
}

pub(crate) struct Service {
    server: server::Comms,
    app: app::Comms
}

impl Service {
    pub fn init() -> Result<Self, &'static str> {
        let server = server::Comms::init().map_err(|_| "server err")?;
        let app = app::Comms::init().map_err(|_| "app err")?;

        Ok(Self { server, app })
    }
}