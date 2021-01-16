pub struct Config {
    pub greeting: String,
    pub NUMBER: i32,
}


impl Config {
    pub fn default() -> Self {
        Config {
            greeting: String::from("Hello, world!"),
            NUMBER: 5,
        }
    }

    pub fn print(&self) {
        println!("Config.greeting = {}", self.greeting);
        println!("Config.NUMBER = {}", self.NUMBER);
    }
}