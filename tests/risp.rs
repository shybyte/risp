extern crate risp;

use risp::*;
use risp::types::RispType::*;
use risp::types::error_result;
use risp::core::create_core_environment;

#[test]
fn test_sum() {
    assert_eq!(eval_risp(r"
        (+ 1
            (* 2 3)
        )
    "), Ok(Int(7)));
}

#[test]
fn test_def() {
    let mut env = create_core_environment();

    assert_eq!(eval_risp_for_env(r"
        (def variable
            (* 2 3)
        )
    ", &mut env), Ok(Int(6)));

    assert_eq!(env.get("variable"), Some(Int(6)));

}

#[test]
fn test_parsing_errors() {
    assert!(eval_risp("(").is_err());
    assert!(eval_risp(")").is_err());
    assert!(eval_risp("").is_err());
}

#[test]
fn test_eval_errors() {
    assert!(eval_risp("()").is_err());
    assert!(eval_risp("(1 2)").is_err());
    assert!(eval_risp("(a 2)").is_err());
    assert!(eval_risp("(def)").is_err());
    assert!(eval_risp("(def a)").is_err());
    assert!(eval_risp("(def 1 2)").is_err());
}

#[test]
fn test_eval_error_expected_function() {
    let mut env = create_core_environment();
    env.set("var", Int(1));
    let result = eval_risp_for_env("(var 1 2 3)", &mut env);
    assert_eq!(result, error_result("Expected function but got Int(1)"));
}