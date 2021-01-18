//! A place for library configuration
//! 
//! PUT GENERAL LIB CONFIG HERE AND CREATE SUBMODULES FOR SPECIFIC THINGS


use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    path::PathBuf,
};
use log::info;

use crate::utils::path_from_cwd;


/// Holds the primary library configuration.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub greeting: String,
    pub number: i32,
}


impl Config {
    /// Initialize config object with default preset.
    pub fn default() -> Self {
        let v = vec![String::from("src"), String::from("config"), String::from("default.ron")];
        let path = path_from_cwd(v).expect("RON file");
        Config::from_file(path)
    }

    /// Load a RON config file from any directory.
    pub fn from_file(path: PathBuf) -> Self {
        // open config file
        info!("Loading config from {}", path.display());
        let f = File::open(path).expect("config file at given path");

        // return config object
        ron::de::from_reader(f).expect("well-formatted RON file")
    }

    /// Print (and log) the current configuration.
    pub fn print(&self) {
        info!("Config.greeting = {}", self.greeting);
        info!("Config.number = {}", self.number);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config() {
        // hardcoded config
        let hc = Config {
            greeting: String::from("Hello, world!"),
            number: 7,
        };

        // default ron config
        let rc = Config::default();

        // compare content
        assert_eq!(hc.number, rc.number);
    }
}