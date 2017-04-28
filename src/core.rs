#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use std::iter;
use environment::*;
use types::*;
use types::RispType::*;
use std::cmp;
use std::ops::{Add, Div, Mul, Sub};

type IntOperation = fn(i64, i64) -> i64;

fn sum(args: Vec<RispType>) -> RispResult {
    apply_to_vector(i64::add, &args)
}

fn apply_to_vector(op: IntOperation, vec: &[RispType]) -> RispResult {
    let s: RispResult = vec.get(0).ok_or_else(|| error("Missing first argument")).map(|x| x.clone());
    vec.iter().skip(1).fold(s, |acc, x| apply_to(op, &acc?, x))
}

fn apply_to(op: IntOperation, x1: &RispType, x2: &RispType) -> RispResult {
    match *x1 {
        Int(x1_int) => match *x2 {
            Int(x2_int) => Ok(Int(op(x1_int, x2_int))),
            Vector(ref x2_vec) => apply_to_number_and_vector(op, x1_int, x2_vec),
            _ => error_result(format!("Operation wants an Int or a Vector as second argument but got {:?}", x2))
        },
        Vector(ref x1_vec) => match *x2 {
            Int(x2_int) => apply_to_vector_and_number(op, x1_vec, x2_int),
            Vector(ref x2_vec) => apply_to_vector_and_vector(op, x1_vec, x2_vec),
            _ => error_result(format!("Operation wants an Int or a Vector as second argument but got {:?}", x2))
        },
        _ => error_result(format!("Operation wants an Int or a Vector as first argument but got {:?}", x1))
    }
}

fn apply_to_number_and_vector(op: IntOperation, x_int: i64, xs: &[RispType]) -> RispResult {
    let x = Int(x_int);
    xs.iter().map(|x2| apply_to(op, &x, x2))
        .collect::<Result<_, _>>()
        .map(Vector)
}

fn apply_to_vector_and_number(op: IntOperation, xs: &[RispType], x_int: i64) -> RispResult {
    let x = Int(x_int);
    xs.iter().map(|x2| apply_to(op, x2, &x))
        .collect::<Result<_, _>>()
        .map(Vector)
}

fn apply_to_vector_and_vector(op: IntOperation, xs1: &[RispType], xs2: &[RispType]) -> RispResult {
    let result_len = cmp::max(xs1.len(), xs2.len());
    (0..result_len)
        .map(|i| apply_to(op, &xs1[i % xs1.len()], &xs2[i % xs2.len()]))
        .collect::<Result<_, _>>()
        .map(Vector)
}

fn mul(vec: Vec<RispType>) -> RispResult {
    apply_to_vector(i64::mul, &vec)
}

fn div(vec: Vec<RispType>) -> RispResult {
    apply_to_vector(i64::div, &vec)
}

fn sub(vec: Vec<RispType>) -> RispResult {
    apply_to_vector(i64::sub, &vec)
}

fn rep(args: Vec<RispType>) -> RispResult {
    if let Some((n, elements)) = args.split_first() {
        match *n {
            Int(n2) => {
                Ok(Vector(repeated(elements, n2 as usize)))
            }
            _ => error_result("rep needs an int as first argument")
        }
    } else {
        error_result("rep needs 2 arguments but got 0")
    }
}

fn repeated<T: Clone>(pattern: &[T], times: usize) -> Vec<T> {
    concat(iter::repeat(pattern.to_vec()).take(times).collect())
}

fn concat<T: Clone>(input: Vec<Vec<T>>) -> Vec<T> {
    input.into_iter().flat_map(|x| x).collect()
}


pub fn create_core_environment() -> Environment {
    let mut env = Environment::new();
    env.set("+", Function(sum));
    env.set("*", Function(mul));
    env.set("/", Function(div));
    env.set("-", Function(sub));
    env.set("rep", Function(rep));
    env
}


/* ------------------------------ Tests ----------------------------------------------- */

#[allow(dead_code)]
fn sum2(x1: &RispType, x2: &RispType) -> RispResult {
    apply_to(i64::add, x1, x2)
}


#[test]
fn test_sum() {
    assert_eq!(sum(vec![Int(1), Int(2)]), Ok(Int(3)));
}

#[test]
fn test_sum_number_vector() {
    assert_eq!(sum(vec![Int(20), Vector(vec![Int(1), Int(2)])]), Ok(Vector(vec![Int(21), Int(22)])));
}

#[test]
fn test_sum_number_vector_vector() {
    assert_eq!(sum(vec![Int(20), Vector(vec![Int(1), Int(2)]), Vector(vec![Int(100), Int(200)])]), Ok(Vector(vec![Int(121), Int(222)])));
}

#[test]
fn test_sum_number_nested_vector() {
    assert_eq!(sum(vec![
        Int(20),
        Vector(vec![Int(1), Int(2), Vector(vec![Int(100), Int(200)])]),
    ])
    , Ok(Vector(vec![Int(21), Int(22), Vector(vec![Int(120), Int(220)])])));
}

#[test]
fn test_sum2_number_vector() {
    assert_eq!(sum2(&Int(20), &Vector(vec![Int(1), Int(2)])), Ok(Vector(vec![Int(21), Int(22)])));
}

#[test]
fn test_sum2_vector_number() {
    assert_eq!(sum2(&Vector(vec![Int(1), Int(2)]), &Int(20)), Ok(Vector(vec![Int(21), Int(22)])));
}

#[test]
fn test_sum2_vector_vector() {
    assert_eq!(sum2(
        &Vector(vec![Int(1), Int(2)]),
        &Vector(vec![Int(10), Int(20)])
    ), Ok(Vector(vec![Int(11), Int(22)])));
}

#[test]
fn test_sum2_vector_longer_vector() {
    assert_eq!(sum2(
        &Vector(vec![Int(1), Int(2)]),
        &Vector(vec![Int(10), Int(20), Int(30)])
    ), Ok(Vector(vec![Int(11), Int(22), Int(31)])));
}

#[test]
fn test_sum2_errors() {
    assert_eq!(sum2(&string("string"), &Int(1)), error_result("Operation wants an Int or a Vector as first argument but got Str(\"string\")"));
    assert_eq!(sum2(&Int(1), &string("string")), error_result("Operation wants an Int or a Vector as second argument but got Str(\"string\")"));
    assert_eq!(sum2(&Vector(vec![Int(1)]), &string("string")), error_result("Operation wants an Int or a Vector as second argument but got Str(\"string\")"));
}


#[test]
fn test_add_number_to_vector() {
    assert_eq!(apply_to_number_and_vector(i64::add, 20, &vec![Int(1), Int(2)]), Ok(Vector(vec![Int(21), Int(22)])));
}


#[test]
fn test_sum_errors() {
    assert_eq!(sum(vec![Nil, Nil]), error_result("Operation wants an Int or a Vector as first argument but got Nil"));
}

#[test]
fn test_mul_errors() {
    assert!(mul(vec![List(vec![]), Int(1)]).is_err());
}

#[test]
fn test_div() {
    assert_eq!(div(vec![Int(10), Int(2)]), Ok(Int(5)));
}

#[test]
fn test_sub() {
    assert_eq!(sub(vec![Int(10), Int(2), Int(1)]), Ok(Int(7)));
}


#[test]
fn test_rep() {
    assert_eq!(
    rep(vec![Int(2), Int(3), Int(4)]),
    Ok(Vector(vec![Int(3), Int(4), Int(3), Int(4)]))
    );
}

#[test]
fn test_rep_nested() {
    assert_eq!(
    rep(vec![Int(2), Vector(vec![Int(3), Int(4)])]),
    Ok(Vector(vec![Vector(vec![Int(3), Int(4)]), Vector(vec![Int(3), Int(4)])]))
    );
}

#[test]
fn test_rep_missing_arguments() {
    assert_eq!(rep(vec![]), error_result("rep needs 2 arguments but got 0"));
}

#[test]
fn test_rep_needs_int_as_first_argument() {
    assert_eq!(rep(vec![string("23")]), error_result("rep needs an int as first argument"));
}