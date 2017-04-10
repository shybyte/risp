
#[derive(Debug, PartialEq)]
pub enum MistType {
    Int(i64),
    List(Vec<MistType>)
}