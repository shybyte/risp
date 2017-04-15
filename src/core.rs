#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use std::iter;
use environment::*;
use types::*;
use types::RispType::*;

fn sum(list: Vec<RispType>) -> RispResult {
    let mut s = 0;
    for x in list {
        match x {
            Int(x2) => { s += x2 }
            _ => return error_result("I want ints!")
        }
    }
    Ok(Int(s))
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
fn test_sum_errors() {
    assert!(sum(vec![List(vec![])]).is_err());
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