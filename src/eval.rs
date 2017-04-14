use types::*;
use types::RispType::*;
use environment::*;
use core::create_core_environment;


pub fn eval(ast: RispType, env: &mut Environment) -> RispResult {
    match ast {
        List(list) => {
            let evaluated_list = list.iter()
                .map(|el| eval(el.clone(), env))
                .collect::<Result<Vec<_>, _>>()?;
            let first_element = evaluated_list.first().ok_or_else(|| error("Empty List"))?;
            match *first_element {
                Symbol(ref symbol) => {
                    match symbol.as_ref() {
                        "def" => {
                            let var = evaluated_list.get(1).ok_or_else(|| error("Missing variable in def"))?;
                            match *var {
                                Symbol(ref sym_var) => {
                                    let value = evaluated_list.get(2).ok_or_else(|| error("Missing value in def"))?;
                                    env.insert(sym_var, value.clone());
                                    Ok(value.clone())
                                }
                                _ => error_result(format!("Expected symbol in def but got {:?}", var))
                            }
                        }
                        _ => {
                            let env_value = env.get(symbol).ok_or_else(|| error(format!("Undefined symbol{:?}", symbol)))?;
                            match env_value {
                                Function(function) => function(evaluated_list[1..].to_vec()),
                                _ => error_result(format!("Expected function but got {:?}", env_value))
                            }
                        }
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
    eval(ast, &mut create_core_environment())
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


#[test]
fn test_def() {
    let variable = "variable";
    let variable_value = Int(23);
    let mut env = create_core_environment();

    assert_eq!(eval(List(vec![
        symbol("def"),
        symbol(variable),
        variable_value.clone()
    ]), &mut env), Ok(variable_value.clone()));

    assert_eq!(env.get(variable), Some(variable_value));
}

#[test]
fn test_def_evaluated() {
    let variable = "variable";
    let mut env = create_core_environment();

    assert_eq!(eval(List(vec![
        symbol("def"),
        symbol(variable),
        List(vec![symbol("+"), Int(1), Int(2)])
    ]), &mut env), Ok(Int(3)));

    assert_eq!(env.get(variable), Some(Int(3)));
}