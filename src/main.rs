use rust_boilerplate::{
    config::Config,
    utils::init_logger,
};

use log::{debug, info, warn};


fn main() -> std::io::Result<()> {
    let conf = Config::default();
    conf.print();

    init_logger()?;

    debug!("debug message");
    info!("info message");
    warn!("warn message");

    Ok(())
}
