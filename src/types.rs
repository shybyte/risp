#[derive(Debug, PartialEq)]
pub struct RispError(String);

pub type RispResult = Result<RispType, RispError>;


#[derive(Debug, PartialEq, Clone)]
pub enum RispType {
    Int(i64),
    List(Vec<RispType>),
    Symbol(String),
    Function(fn(Vec<RispType>) -> RispResult),
}


pub fn error<S: Into<String>>(message: S) -> RispError {
    RispError(message.into())
}

pub fn error_result<S: Into<String>>(message: S) -> RispResult {
    Err(error(message))
}