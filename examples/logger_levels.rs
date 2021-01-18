use rust_boilerplate::utils::logging::init_logger;

use log::{debug, info, warn, error};


fn main() -> std::result::Result<(), std::io::Error> {
    init_logger();

    #[cfg(debug_assertions)]
    info!("If you don't want to see debug messages, use the --release version.");

    #[cfg(not(debug_assertions))]
    info!("If you want to see debug messages, use the debug build.");

    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");

    Ok(())
}
