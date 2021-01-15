pub fn init_logger() -> std::result::Result<(), std::io::Error> {
    // import stuff 
    use simplelog::*;
    use std::fs::File;
    use std::time::{SystemTime, UNIX_EPOCH};

    // determine logger level
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;

    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Info;

    
    fn init_combined_logger(level: LevelFilter, file: File) {
        // initialize simplelog terminal and file logger
        CombinedLogger::init(
            vec![
                TermLogger::new(level, Config::default(), TerminalMode::Mixed),
                WriteLogger::new(level, Config::default(), file),
            ]
        ).unwrap();
    }

    // get timestamp
    let stamp: String = match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(n) => n.as_secs().to_string(),
        Err(_) => String::from("0"),
    };

    // generate file name
    let mut log_name = String::from("log");
    log_name.push_str(&stamp);
    log_name.push_str(".txt");

    // generate absolute path
    let mut log_dir = std::env::current_dir()?;
    log_dir = log_dir.join("log").join(log_name);

    println!("Trying to create log file at: {}", log_dir.display());

    // try to create new log file in `./log/`
    // fails if directory doesnt exist and returns error
    match File::create(log_dir){
        Ok(f) => init_combined_logger(level, f),
        Err(_) => TermLogger::init(level, Config::default(), TerminalMode::Mixed).unwrap(),
    };

    Ok(())
}
