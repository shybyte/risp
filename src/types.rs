
#[derive(Debug, PartialEq, Clone)]
pub enum RispType {
    Int(i64),
    List(Vec<RispType>),
    Symbol(String),
}