use regex::Regex;
use types::MistType;
use types::MistType::*;

pub fn parse(input: &str) -> Result<MistType, String> {
    let number_regexp = Regex::new(r"^-?\d+").unwrap();
    if let Some(cap)  = number_regexp.captures(input) {
        Ok(Int(cap[0].parse().unwrap()))
    } else {
        Err("Error".to_string())
    }
}


#[test]
fn parse_number() {
    assert_eq!(parse("0"), Ok(Int(0)));
    assert_eq!(parse("1"), Ok(Int(1)));
    assert_eq!(parse("42"), Ok(Int(42)));
    assert_eq!(parse("-42"), Ok(Int(-42)));
}
