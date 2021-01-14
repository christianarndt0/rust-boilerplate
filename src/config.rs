pub struct Config {
    pub greeting: String,
}


impl Config {
    pub fn default() -> Self {
        Config {
            greeting: String::from("Hello, world!"),
        }
    }

    pub fn print(&self) {
        println!("Config.greeting = {}", self.greeting);
    }
}