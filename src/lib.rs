extern crate regex;

pub mod core;
pub mod environment;
pub mod eval;
pub mod parse;
mod tokenize;
pub mod types;

use types::RispResult;
use parse::parse;
use eval::eval;
use environment::Environment;
use core::create_core_environment;

pub fn eval_risp(risp_code: &str) -> RispResult {
    eval_risp_for_env(risp_code, &mut create_core_environment())
}

pub fn eval_risp_for_env(risp_code: &str, env: &mut Environment) -> RispResult {
    let ast = parse(risp_code)?;
    eval(ast, env)
}