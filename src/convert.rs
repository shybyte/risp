use types::RispType;
use types::RispType::*;
use types::RispError;
use types::*;
use std::collections::HashMap;


impl Into<Result<RispType, RispError>> for RispType {
    fn into(self) -> Result<RispType, RispError> {
        Ok(self)
    }
}


impl Into<Result<i64, RispError>> for RispType {
    fn into(self) -> Result<i64, RispError> {
        match self {
            Int(int) => Ok(int),
            _ => Err(error(format!("Expected Int but got {:?}", self))),
        }
    }
}

impl Into<Result<String, RispError>> for RispType {
    fn into(self) -> Result<String, RispError> {
        match self {
            Str(s) => Ok(s),
            _ => Err(error(format!("Expected String but got {:?}", self))),
        }
    }
}

impl Into<Result<HashMap<String, RispType>, RispError>> for RispType {
    fn into(self) -> Result<HashMap<String, RispType>, RispError> {
        match self {
            Map(map) => Ok(map),
            _ => Err(error(format!("Expected Map but got {:?}", self)))
        }
    }
}

impl Into<Result<Vec<RispType>, RispError>> for RispType {
    fn into(self) -> Result<Vec<RispType>, RispError> {
        match self {
            Vector(vector) => Ok(vector),
            _ => Err(error(format!("Expected Vector but got {:?}", self)))
        }
    }
}

impl Into<Result<Vec<i64>, RispError>> for RispType {
    fn into(self) -> Result<Vec<i64>, RispError> {
        let vec_of_risp: Result<Vec<RispType>, _> = self.into();
        vec_of_risp?.iter().cloned()
            .map(|el| el.into())
            .collect::<Result<Vec<i64>, _>>()
    }
}


impl RispType {
    pub fn get<T>(&self, key: &str) -> Result<Option<T>, RispError> where RispType: Into<Result<T, RispError>> {
        match *self {
            Map(ref map) => {
                match map.get(key).cloned() {
                    Some(risp_value) => {
                        Ok(Some(risp_value.into()?))
                    }
                    None => Ok(None)
                }
            }
            _ => Err(error(format!("Expected Map but got {:?}", self)))
        }
    }
}


pub fn flatten_into<T>(risp_vec_input: RispType) -> Result<Vec<T>, RispError>
    where RispType: Into<Result<T, RispError>> {
    match risp_vec_input {
        Vector(vector) => {
            let flat = flatten_vec(vector);
            flat.iter().cloned()
                .map(|el| el.into())
                .collect()
        }
        _ => Err(error(format!("Expected Vector but got {:?}", risp_vec_input)))
    }
}

pub fn flatten_vec(risp_vec: Vec<RispType>) -> Vec<RispType> {
    let mut result = vec![];
    for el in risp_vec {
        if let RispType::Vector(vector) = el {
            for child_el in flatten_vec(vector) {
                result.push(child_el)
            }
        } else {
            result.push(el);
        }
    }
    result
}


/* ------------------------------ Tests ----------------------------------------------- */


#[test]
fn test_convert_int() {
    let result: Result<i64, _> = Int(3).into();
    assert_eq!(result, Ok(3));
}

#[test]
fn test_convert_int_error() {
    let result: Result<i64, RispError> = List(vec![]).into();
    assert!(result.is_err());
}


#[test]
fn test_convert_string() {
    let result: Result<String, _> = string("string").into();
    assert_eq!(result, Ok("string".to_string()));
}

#[test]
fn test_convert_string_error() {
    let result: Result<String, RispError> = List(vec![]).into();
    assert!(result.is_err());
}

#[test]
fn test_convert_vector() {
    let result: Result<Vec<_>, RispError> = Vector(vec![Int(23)]).into();
    assert_eq!(result, Ok(vec![Int(23)]));
}

#[test]
fn test_convert_vector_error() {
    let result: Result<Vec<RispType>, RispError> = Int(1).into();
    assert!(result.is_err());
}

#[test]
fn test_convert_vector_int() {
    let result: Result<Vec<i64>, RispError> = Vector(vec![Int(23)]).into();
    assert_eq!(result, Ok(vec![23]));
}

#[test]
fn test_convert_map() {
    let input_map = map(vec![
        ("key", Int(23))
    ]);
    let result: Result<HashMap<String, RispType>, RispError> = input_map.into();
    assert_eq!(result.unwrap().get("key").unwrap().clone(), Int(23));
}

#[test]
fn test_convert_map_error() {
    let result: Result<HashMap<String, RispType>, RispError> = List(vec![]).into();
    assert_eq!(result, Err(error(format!("Expected Map but got List([])"))));
}

#[test]
fn test_get() {
    let input_map = map(vec![
        ("key", Int(23))
    ]);
    let int_option = input_map.get("key").unwrap();
    assert_eq!(int_option, Some(23));
}

#[test]
fn test_get_risptype() {
    let input_map = map(vec![
        ("key", Int(23))
    ]);
    let int_option: Option<RispType> = input_map.get("key").unwrap();
    assert_eq!(int_option, Some(Int(23)));
}

#[test]
fn test_get_none() {
    let input_map = map(vec![
        ("key", Int(23))
    ]);
    let int_option: Option<i64> = input_map.get("unknown_key").unwrap();
    assert_eq!(int_option, None);
}

#[test]
fn test_get_error_expected_int() {
    let input_map = map(vec![
        ("key", string("string"))
    ]);
    let int_result: Result<Option<i64>, _> = input_map.get("key");
    assert_eq!(int_result, Err(error(format!("Expected Int but got {:?}", string("string")))));
}

#[test]
fn test_get_error_expected_map() {
    let input = Int(123);
    let int_result: Result<Option<i64>, _> = input.get("key");
    assert_eq!(int_result, Err(error(format!("Expected Map but got Int(123)"))));
}


#[test]
fn test_flatten_vec() {
    let input = vec![
        Int(1),
        Vector(vec![Int(2), Int(3)])
    ];
    let flat_result = flatten_vec(input);
    assert_eq!(flat_result, vec![Int(1), Int(2), Int(3)] );
}

#[test]
fn test_flatten_into() {
    let input = Vector(vec![
        Int(1),
        Vector(vec![Int(2), Int(3)])
    ]);
    let flat_result = flatten_into(input);
    assert_eq!(flat_result, Ok(vec![1, 2, 3]));
}

#[test]
fn test_flatten_error_outer() {
    let flat_result: Result<Vec<i64>, _> = flatten_into(Int(1));
    assert_eq!(flat_result, Err(error("Expected Vector but got Int(1)")));
}

#[test]
fn test_flatten_error_inner() {
    let input = Vector(vec![
        Int(1),
        Vector(vec![string("string"), Int(3)])
    ]);
    let flat_result: Result<Vec<i64>, _> = flatten_into(input);
    assert_eq!(flat_result, Err(error("Expected Int but got Str(\"string\")")));
}
