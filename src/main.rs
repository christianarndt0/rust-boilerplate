use rust_boilerplate::{
    config::Config,
    utils::init_logger,
};

use log::{debug, info, warn};


fn main() -> std::result::Result<(), std::io::Error> {
    let conf = Config::default();
    conf.print();

    // must have ./log/ dir for debug build
    init_logger()?;

    debug!("debug message");
    info!("info message");
    warn!("warn message");

    Ok(())
}
