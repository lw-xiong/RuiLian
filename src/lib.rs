#![allow(dead_code)]
#![allow(unused_variables)]
pub mod ast;
pub mod environment;
pub mod interpreter;
pub(crate) mod parser;
pub(crate) mod scanner;
pub mod tokens;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
