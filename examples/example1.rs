use rust_boilerplate::{
    config::Config,
    utils::logging::init_logger,
};

use log::{debug, info, warn, error};


fn main() -> std::result::Result<(), std::io::Error> {
    let conf = Config::default();
    conf.print();

    init_logger()?;

    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");

    Ok(())
}
