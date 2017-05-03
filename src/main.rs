#[macro_use] extern crate lazy_static;
extern crate regex;

mod core;
mod environment;
mod eval;
mod parse;
mod types;
mod tokenize;

use std::fs::File;
use std::io::prelude::*;
use parse::parse;
use eval::eval;
use core::create_core_environment;


fn main() {
    let mut file = File::open("examples/kitchen_sink.risp").unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).unwrap();

    let ast = parse(&("(do ".to_string() + &risp_code + ")")).unwrap();
    let result = eval(ast, &mut create_core_environment());
    println!("{:?}", result);
}

