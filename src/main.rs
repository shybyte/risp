extern crate regex;

mod core;
mod environment;
mod eval;
mod parse;
mod types;

use std::fs::File;
use std::io::prelude::*;
use parse::parse;
use eval::eval;
use core::create_core_environment;


fn main() {
    let mut file = File::open("examples/simple.risp").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //let wrapped_content = "(".to_string() + &contents + ")";

    let ast = parse(&contents).unwrap();
    println!("{:?}", ast);
    let result = eval(ast, &mut create_core_environment());
    println!("{:?}", result);

}

