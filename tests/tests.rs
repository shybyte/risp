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