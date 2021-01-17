use rust_boilerplate::{
    config::Config,
    utils::logging::init_logger,
};

use log::info;


fn main() -> Result<(), std::io::Error> {
    init_logger();

    let conf = Config::default();
    conf.print();

    Ok(())
}
