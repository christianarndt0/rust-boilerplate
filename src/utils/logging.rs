use simplelog::*;
use log::{info, warn};


fn init_combined_logger(level: LevelFilter) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::path::PathBuf;

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

    info!("Initialized terminal and file logger at {}", log_dir_string);

    Ok(())
}


fn init_terminal_logger(level: LevelFilter) {
    TermLogger::init(level, Config::default(), TerminalMode::Mixed).expect("simplelog terminal logger");

    warn!("Unable to initialize file logger, only initializing terminal logger");
}


/// initialize a simplelog terminal logger and optionally a file logger if ./log/ exists
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
