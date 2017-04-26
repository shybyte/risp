use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct RispError(String);

pub type RispResult = Result<RispType, RispError>;


#[derive(Debug, PartialEq, Clone)]
pub enum RispType {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    List(Vec<RispType>),
    Vector(Vec<RispType>),
    Map(HashMap<String, RispType>),
    Keyword(String),
    Symbol(String),
    Function(fn(Vec<RispType>) -> RispResult),
}


pub fn error<S: Into<String>>(message: S) -> RispError {
    RispError(message.into())
}

pub fn error_result<S: Into<String>>(message: S) -> RispResult {
    Err(error(message))
}

pub fn symbol<S: Into<String>>(s: S) -> RispType {
    RispType::Symbol(s.into())
}

pub fn keyword<S: Into<String>>(s: S) -> RispType {
    RispType::Keyword(s.into())
}

pub fn string<S: Into<String>>(s: S) -> RispType {
    RispType::Str(s.into())
}

#[allow(dead_code)]
pub fn map<S: Into<String>>(pairs: Vec<(S, RispType)>) -> RispType {
    let result: HashMap<String, RispType> = pairs.into_iter()
        .map(|(s, r)| (s.into(), r))
        .collect();
    RispType::Map(result)
}