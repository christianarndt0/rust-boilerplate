//! Basic utilities module
//! 
//! This module contains a sample function and test (add) that serve as a template and should be replaced.
//! 
//! The imported logging submodule is fully functional and can be used as is in any project.

pub mod logging;


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    // import everything from the parent (utils) module
    use super::*;

    // define test
    #[test]
    fn test_add() {
        assert_eq!(4, add(2, 2));
    }
}