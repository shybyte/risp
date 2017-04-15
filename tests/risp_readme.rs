extern crate risp;

use risp::eval_risp_script;
use risp::types::RispType::Int;
use risp::core::create_core_environment;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_minimal_example() {
    let mut env = create_core_environment();
    env.set("var", Int(2));

    let risp_script = "(+ 40 var)";
    let result = eval_risp_script(risp_script, &mut env);

    assert_eq!(result, Ok(Int(42)));
}


#[test]
fn test_kitchen_sink() {
    let mut file = File::open("examples/kitchen_sink.risp").unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).unwrap();

    let mut env = create_core_environment();
    let result = eval_risp_script(&risp_code, &mut env);
    assert!(result.is_ok());
}