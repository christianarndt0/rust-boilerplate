//! Basic utilities module
//! 
//! PUT SMALL MISC STUFF IN HERE THAT DOESN'T NECESSARILY FIT ANYWHERE ELSE


pub mod logging;


/// Wait for console input that is either `y` (+ Return) or `n` (+ Return)
pub fn ask_yesno() -> bool {
    // init empty string for console input
    let mut line = String::new();

    // read line from console
    std::io::stdin().read_line(&mut line).unwrap();

    // return result or ask again if input is neither y nor n
    match &line[..2] {
        "y\n" => true,
        "n\n" => false,
        _     => ask_yesno(),
    }
}