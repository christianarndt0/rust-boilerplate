use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    path::PathBuf,
};
use log::{info, error};


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub greeting: String,
    pub number: i32,
}


impl Config {
    pub fn default() -> Self {
        let mut path = std::env::current_dir().expect("absolute directory of executable");
        path = path.join("src/config/default.ron");
        Config::from_file(path)
    }

    pub fn from_file(path: PathBuf) -> Self {
        // open config file
        info!("Trying to load config from {}", path.display());
        let f = File::open(path).expect("config file at given path");

        // return config object
        ron::de::from_reader(f).expect("well-formatted RON file")
    }

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