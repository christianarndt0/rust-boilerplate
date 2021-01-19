use rust_boilerplate::{
    Config,
    utils::logging::init_logger,
};


fn main() -> Result<(), std::io::Error> {
    let conf = Config::new();

    init_logger(&conf.logger_cfg);
    log::info!("logger_cfg: {}", conf.logger_cfg);

    Ok(())
}
