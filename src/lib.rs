//! This will not compile because the macro is expecting a
//! string literal
//!
//! ```compile_fail
//! let hello_c = "Hello c!";
//! let res = macro_parser::parse!(hello_c);
//!
//! ```
pub use macro_parser::*;
pub use parser::*;

#[cfg(test)]
mod tests {
    use parser::Hello;

    #[test]
    fn test() {
        let res = parser::parser("Hello a!").unwrap();
        println!("res: {:?}", res);

        let res = macro_parser::parse!("Hello b!");
        println!("res: {:?}", res);

        // won't compile
        //let res = macro_parser::parse!(123);
        //let res = macro_parser::parse!(hello_c);
    }
}
