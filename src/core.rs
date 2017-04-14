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


pub fn create_core_environment() -> Environment {
    let mut env = Environment::new();
    env.set("+", Function(sum));
    env.set("*", Function(mul));
    env
}




/* Tests */

#[test]
fn test_sum_errors() {
    assert!(sum(vec![List(vec![])]).is_err());
}

#[test]
fn test_mul_errors() {
    assert!(mul(vec![List(vec![])]).is_err());
}