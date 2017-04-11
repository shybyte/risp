
#[derive(Debug, PartialEq, Clone)]
pub enum MistType {
    Int(i64),
    List(Vec<MistType>),
    Symbol(String)
}