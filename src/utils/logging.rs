//! Convenience methods to initialize logging
//! 
//! Uses rusts logging facade (log crate) for logging macros and the simplelog crate to handle terminal and file logging.


use simplelog::*;
use log::{info, error};
use std::str::FromStr;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggerConfig {
    pub output_dir: String,
    pub level: String,
}


impl std::fmt::Display for LoggerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(output_dir: {}, level: {})", self.output_dir, self.level)
    }
}


/// Initialize a simplelog terminal logger and optionally a file logger if the configured directory exists.  
pub fn init_logger(cfg: &LoggerConfig) {
    // determine logger level
    let level = LevelFilter::from_str(&cfg.level).expect("init LevelFilter from string");

    // check if log directory exists
    let log_dir = std::path::Path::new(&cfg.output_dir);

    if log_dir.is_dir() {
        // try to initialize combined logger, otherwise only initialize terminal logger
        match init_combined_logger(&log_dir, level) {
            Ok(_) => (),
            Err(_) => init_terminal_logger(cfg, level),
        }   
    }
    else {
        error!("output dir {} is not a directory", cfg.output_dir);
        init_terminal_logger(cfg, level);
    }
}


fn init_combined_logger(log_dir: &std::path::Path, level: LevelFilter) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::time::{SystemTime, UNIX_EPOCH};

    // get timestamp
    let stamp: String = match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(n) => n.as_secs().to_string(),
        Err(_) => String::from("0"),
    };

    // generate file name
    let log_name = format!("log{}.txt", &stamp);

    // cat log directory and file name
    let pb = std::path::PathBuf::from(log_dir).join(log_name);

    // create file
    let file = File::create(&pb)?;

    // init simplelog logger
    CombinedLogger::init(
        vec![
            TermLogger::new(level, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(level, Config::default(), file),
        ]
    ).expect("simplelog combined logger");

    info!("Initialized terminal logger");
    info!("Initialized file logger at {}", pb.display());

    Ok(())
}


fn init_terminal_logger(_cfg: &LoggerConfig, level: LevelFilter) {
    TermLogger::init(level, Config::default(), TerminalMode::Mixed).expect("simplelog terminal logger");

    info!("Initialized terminal logger");

    // only warn about missing log directory in debug builds
    #[cfg(debug_assertions)]
    log::error!("Unable to initialize file logger, check if {} exists and is a directory", _cfg.output_dir);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let cfg = LoggerConfig {
            output_dir: String::from("this/does/not/exist/and/would/be/a/file.txt"),
            level: String::from("Debug"),
        };

        // should (gracefully) fail to init a file logger but still initialize a terminal logger
        init_logger(&cfg);

        // not panicking is good enough
        assert!(true);
    }
}