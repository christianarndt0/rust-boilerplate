use {{crate_name}}::{
    Config,
    utils::ask_yesno,
    utils::logging::init_logger,
};

use log::{debug, info, warn, error};


#[test]
fn test_logger_levels() {
    // create default configuration
    let cfg = Config::new();

    // initialize logger
    init_logger(&cfg.logger_cfg);

    // print messages depending on log level in config
    let mut q = String::from("Is this a nice display of log messages >= ");
    q.push_str(&cfg.logger_cfg.level);
    q.push_str("? (y/n):");

    debug!("{}", q);
    info!("{}", q);
    warn!("{}", q);
    error!("{}", q);

    // this is a manual test, ask human if test passed
    assert!(ask_yesno());
}