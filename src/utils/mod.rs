pub fn init_logger() -> std::result::Result<(), std::io::Error> {
    // import stuff 
    use simplelog::*;
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

    // generate absolute path
    let mut log_dir = std::env::current_dir()?;
    log_dir = log_dir.join("log").join(log_name);

    println!("Trying to create log file at: {}", log_dir.display());

    // initialize simplelog terminal and file logger
    #[cfg(debug_assertions)]
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(log_dir).unwrap()),
        ]
    ).unwrap();

    #[cfg(not(debug_assertions))]
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log_dir).unwrap()),
        ]
    ).unwrap();

    Ok(())
}
