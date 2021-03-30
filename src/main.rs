use std::fs;

use parser::parse;
use types::type_check_function;

mod parser;
mod types;
mod scope;

fn main() {
    let source = fs::read_to_string("test.txt").unwrap();
    let function = parse(&source);
    println!("{:#?}", function);
    let scope = type_check_function(&function);
    println!("{:#?}", scope);
}
