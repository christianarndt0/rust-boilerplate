//! Convenience methods to initialize logging
//! 
//! Uses rusts logging facade (log crate) for logging macros and the simplelog crate to handle terminal and file logging.
//! 
//! If a ./log/ directory exists, a new timestamped log file will be created on each run.  
//! If the directory doesn't exist, log statements will only be printed in the console.


use simplelog::*;
use log::{info, error};


/// Initialize a simplelog terminal logger and optionally a file logger if ./log/ exists.  
/// Logger level: debug for non-release builds and info for release builds.
pub fn init_logger() {
    // determine logger level
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;

    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Info;

    // try to initialize combined logger, otherwise only initialize terminal logger
    match init_combined_logger(level) {
        Ok(_) => (),
        Err(_) => init_terminal_logger(level),
    }
}


fn init_combined_logger(level: LevelFilter) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::time::{SystemTime, UNIX_EPOCH};

    // get timestamp
    let stamp: String = match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(n) => n.as_secs().to_string(),
        Err(_) => String::from("0"),
    };

    // generate file name
    let mut log_name = String::from("log");
    log_name.push_str(&stamp);
    log_name.push_str(".txt");

    // get absolute path of ./log/logTIMESTAMP.txt
    let v = vec![String::from("log"), log_name];
    let log_dir = super::path_from_cwd(v)?;
    let log_dir_string = log_dir.to_string_lossy().to_string();

    // create file
    let file = File::create(log_dir)?;

    // init simplelog logger
    CombinedLogger::init(
        vec![
            TermLogger::new(level, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(level, Config::default(), file),
        ]
    ).expect("simplelog combined logger");

    info!("Initialized terminal logger");
    info!("Initialized file logger at {}", log_dir_string);

    Ok(())
}


fn init_terminal_logger(level: LevelFilter) {
    TermLogger::init(level, Config::default(), TerminalMode::Mixed).expect("simplelog terminal logger");

    info!("Initialized terminal logger");

    // only warn about missing log directory in debug builds
    #[cfg(debug_assertions)]
    error!("Unable to initialize file logger, check if ./log/ exists");
}
