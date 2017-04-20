use types::RispType;
use types::RispType::*;
use types::RispError;
use types::*;
use std::collections::HashMap;


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


fn into_vec(risp_value: RispType) -> Result<Vec<RispType>, RispError> {
    match risp_value {
        Vector(vector) => Ok(vector),
        _ => Err(error(format!("Expected Vector but got {:?}", risp_value)))
    }
}

impl Into<Result<Vec<i64>, RispError>> for RispType {
    fn into(self) -> Result<Vec<i64>, RispError> {
        into_vec(self)?.iter().cloned()
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
    assert!(result.is_err());
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
fn test_get_error() {
    let input_map = map(vec![
        ("key", string("string"))
    ]);
    let int_result: Result<Option<i64>, _> = input_map.get("key");
    assert!(int_result.is_err());
}
