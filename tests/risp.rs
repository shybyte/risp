extern crate risp;

use risp::eval_risp;
use risp::types::RispType::*;

#[test]
fn test_sum() {
    assert_eq!(eval_risp(r"
        (+ 1
            (* 2 3)
        )
    "), Ok(Int(7)));
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
}
