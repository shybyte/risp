use std::collections::HashMap;
use types::*;
use types::RispType::*;
use environment::*;
use core::create_core_environment;


pub fn eval(ast: RispType, env: &mut Environment) -> RispResult {
    match ast {
        List(list) => {
            let first_element = list.first().ok_or_else(|| error("Empty List"))?;
            match *first_element {
                Symbol(ref symbol) => {
                    match symbol.as_ref() {
                        "def" => {
                            let var = list.get(1).ok_or_else(|| error("Missing variable in def"))?;
                            match *var {
                                Symbol(ref sym_var) => {
                                    let value_ast = list.get(2).ok_or_else(|| error("Missing value in def"))?;
                                    let value = eval(value_ast.clone(), env)?;
                                    env.set(sym_var, value.clone());
                                    Ok(value)
                                }
                                _ => error_result(format!("Expected symbol in def but got {:?}", var))
                            }
                        },
                        "do" => {
                            if let Some((last, elements)) =  (&list[1..]).split_last() {
                                for child_ast in elements.iter() {
                                    eval(child_ast.clone(), env)?;
                                }
                                eval(last.clone(), env)
                            } else {
                                error_result("Empty do block")
                            }
                        }
                        _ => {
                            let evaluated_tail = list[1..].iter()
                                .map(|el| eval(el.clone(), env))
                                .collect::<Result<Vec<_>, _>>()?;
                            let env_value = env.get(symbol).ok_or_else(|| error(format!("Undefined symbol{:?}", symbol)))?;
                            match env_value {
                                Function(function) => function(evaluated_tail.to_vec()),
                                _ => error_result(format!("Expected function but got {:?}", env_value))
                            }
                        }
                    }
                }
                _ => error_result(format!("Expected symbol but got {:?}", first_element))
            }
        }
        Vector(vector) => {
            let evaluated_vector = vector.iter()
                .map(|el| eval(el.clone(), env))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Vector(evaluated_vector))
        }

        Map(map_value) => {
            let evaluated_map = map_value.iter()
                .map(|(key, val)|
                    eval(val.clone(), env).map(|evaluated_value|
                        (key.to_string(), evaluated_value)))
                .collect::<Result<HashMap<String, RispType>, _>>()?;
            Ok(Map(evaluated_map))
        }

        Symbol(symbol) => {
            env.get(&symbol).ok_or_else(|| error(format!("symbol '{:?}' is undefined", symbol)))
        }
        other => Ok(other)
    }
}


/* ------------------------------ Tests ----------------------------------------------- */

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

#[test]
fn test_eval_simple_vector() {
    let simple_vector = Vector(vec![Int(1), Int(2)]);
    assert_eq!(eval_test(simple_vector.clone()), Ok(simple_vector));
}

#[test]
fn test_eval_nested_vector() {
    let simple_vector = Vector(vec![Int(1), List(vec![symbol("+"), Int(1), Int(2)])]);
    assert_eq!(eval_test(simple_vector.clone()), Ok(Vector(vec![Int(1), Int(3)])));
}


#[test]
fn test_eval_simple_map() {
    let simple_map = map(vec![
        ("key1", Int(1)),
        ("key2", Int(2))
    ]);
    assert_eq!(eval_test(simple_map.clone()), Ok(simple_map));
}

#[test]
fn test_eval_nested_map() {
    let input_map = map(vec![
        ("key", List(vec![symbol("+"), Int(1), Int(2)]))
    ]);
    let expected_output_map = map(vec![
        ("key", Int(3))
    ]);
    assert_eq!(eval_test(input_map.clone()), Ok(expected_output_map));
}
