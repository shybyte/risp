use types::*;
use types::RispType::*;
use environment::*;
use core::create_core_environment;


pub fn eval(ast: RispType, env: &Environment) -> RispResult {
    match ast {
        List(list) => {
            let evaluated_list = list.iter()
                .map(|el| eval(el.clone(), env))
                .collect::<Result<Vec<_>, _>>()?;
            let first_element = evaluated_list.first().ok_or_else(|| error("Empty List"))?;
            match *first_element {
                Symbol(ref symbol) => {
                    let env_value = env.get(symbol);
                    match env_value {
                        Function(function) => function(evaluated_list[1..].to_vec()),
                        _ => error_result(format!("Expected function but got {:?}", env_value))
                    }
                }
                _ => error_result(format!("Expected symbol but got {:?}", first_element))
            }
        }
        other => Ok(other)
    }
}


#[allow(dead_code)]
fn eval_test(ast: RispType) -> RispResult {
    eval(ast, &create_core_environment())
}

#[test]
fn test_eval_number() {
    assert_eq!(eval_test(Int(23)), Ok(Int(23)));
}

#[test]
fn test_eval_math() {
    assert_eq!(eval_test(List(vec![Symbol("+".to_string()), Int(1), Int(2)])), Ok(Int(3)));
}

#[test]
fn test_nested_math() {
    assert_eq!(eval_test(List(vec![
        Symbol("+".to_string()), Int(1),
        List(vec![Symbol("+".to_string()), Int(10), Int(100)])
    ])), Ok(Int(111)));
}

#[test]
fn test_mul() {
    assert_eq!(eval_test(List(vec![
        Symbol("+".to_string()), Int(1),
        List(vec![Symbol("*".to_string()), Int(10), Int(23)])
    ])), Ok(Int(231)));
}