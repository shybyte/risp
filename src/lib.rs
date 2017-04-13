extern crate regex;

pub mod core;
pub mod environment;
pub mod eval;
pub mod parse;
pub mod types;

use types::RispResult;
use parse::parse;
use eval::eval;
use core::create_core_environment;

pub fn eval_risp(risp_code: &str) -> RispResult {
    let ast = parse(&risp_code)?;
    eval(ast, &create_core_environment())
}