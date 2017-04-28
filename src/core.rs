#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use std::iter;
use environment::*;
use types::*;
use types::RispType::*;
use std::cmp;

fn sum(list: Vec<RispType>) -> RispResult {
    list.iter().fold(Ok(Int(0)), |acc, x| sum2(&acc?, x))
}

fn sum2(x1: &RispType, x2: &RispType) -> RispResult {
    match *x1 {
        Int(x1_int) => match *x2 {
            Int(x2_int) => Ok(Int(x1_int + x2_int)),
            Vector(ref x2_vec) => add_number_to_vector(x1_int, x2_vec),
            _ => error_result(format!("Sum wants an Int or a Vector as second argument but got {:?}", x2))
        },
        Vector(ref x1_vec) => match *x2 {
            Int(x2_int) => add_number_to_vector(x2_int, x1_vec),
            Vector(ref x2_vec) => add_vector_to_vector(x1_vec, x2_vec),
            _ => error_result(format!("Sum wants an Int or a Vector as second argument but got {:?}", x2))
        },
        _ => error_result(format!("Sum wants an Int or a Vector as first argument but got {:?}", x1))
    }
}

fn add_number_to_vector(x_int: i64, xs: &Vec<RispType>) -> RispResult {
    let x = Int(x_int);
    xs.iter().map(|x2| sum2(&x, x2))
        .collect::<Result<_, _>>()
        .map(Vector)
}

fn add_vector_to_vector(xs1: &Vec<RispType>, xs2: &Vec<RispType>) -> RispResult {
    let result_len = cmp::max(xs1.len(), xs2.len());
    (0..result_len)
        .map(|i| sum2(&xs1[i % xs1.len()], &xs2[i % xs2.len()]))
        .collect::<Result<_, _>>()
        .map(Vector)
}

fn mul(list: Vec<RispType>) -> RispResult {
    let mut s = 1;
    for x in list {
        match x {
            Int(x2) => { s *= x2 }
            _ => return error_result("I want ints!")
        }
    }
    Ok(Int(s))
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
    env.set("rep", Function(rep));
    env
}


/* ------------------------------ Tests ----------------------------------------------- */

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
    assert_eq!(sum2(&string("string"), &Int(1)), error_result("Sum wants an Int or a Vector as first argument but got Str(\"string\")"));
    assert_eq!(sum2(&Int(1), &string("string")), error_result("Sum wants an Int or a Vector as second argument but got Str(\"string\")"));
    assert_eq!(sum2(&Vector(vec![Int(1)]), &string("string")), error_result("Sum wants an Int or a Vector as second argument but got Str(\"string\")"));
}


#[test]
fn test_add_number_to_vector() {
    assert_eq!(add_number_to_vector(20, &vec![Int(1), Int(2)]), Ok(Vector(vec![Int(21), Int(22)])));
}


#[test]
fn test_sum_errors() {
    assert!(sum(vec![Nil]).is_err());
}

#[test]
fn test_mul_errors() {
    assert!(mul(vec![List(vec![])]).is_err());
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