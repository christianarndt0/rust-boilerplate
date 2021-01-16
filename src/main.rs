use rust_boilerplate::{
    config::Config,
    utils::logging::init_logger,
};


fn main() -> Result<(), std::io::Error> {
    let conf = Config::default();
    conf.print();

    init_logger()?;

    Ok(())
}
