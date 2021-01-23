use {{crate_name}}::{
    Config,
    utils::logging::init_logger,
};


fn main() -> Result<(), std::io::Error> {
    // create default config
    let conf = Config::new();

    // initialize logger and print its config
    init_logger(&conf.logger_cfg);
    log::info!("logger_cfg: {}", conf.logger_cfg);

    Ok(())
}