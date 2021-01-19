//! lib.rs
//! 
//! DECLARE LIBRARY MODULES AND DO VERY TOP-LEVEL LIB STUFF IN HERE


// declare library modules
pub mod utils;


// import libraries
use std::path::Path;


/// Holds the primary library configuration.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub logger_cfg: utils::logging::LoggerConfig,
}


impl Config {
    /// Initialize config object with `debug()` by default or `release()` when `--release` build option is used.
    pub fn new() -> Self {
        if cfg!(debug_assertions){
            Config::debug()
        }
        else {
            Config::release()
        }
    }

    /// Initialize config object with debug preset from `./config/debug.ron`.
    pub fn debug() -> Self {
        let path = Path::new("./config/debug.ron");
        Config::from_file(&path)
    }

    /// Initialize config object with release preset from `./config/release.ron`.
    pub fn release() -> Self {
        let path = Path::new("./config/release.ron");
        Config::from_file(&path)
    }

    /// Load a RON config file from any directory.
    pub fn from_file(path: &Path) -> Self {
        // open config file
        let f = std::fs::File::open(path).expect("opening config file at given path");

        // return config object
        ron::de::from_reader(f).expect("loading well-formatted RON file")
    }
}