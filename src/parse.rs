use regex::Regex;
use types::MistType;
use types::MistType::*;
//use std::iter::Peekable;

#[derive(Debug, PartialEq,Copy,Clone)]
enum TokenType {
    Number,
    ListStart,
    ListEnd
}

struct Tokenizer {
    input: String,
    pos: usize
}

impl Tokenizer {
    fn new(input: &str) -> Tokenizer {
        Tokenizer { input: input.to_string(), pos: 0 }
    }
}

impl Iterator for Tokenizer {
    type Item = (TokenType, String);

    fn next(&mut self) -> Option<(TokenType, String)> {
        let input = &self.input[self.pos..];
        let input_trimmed = input.trim_left();
        self.pos += input.len() - input_trimmed.len();
        let input = input_trimmed;

        if input.starts_with('(') {
            self.pos += 1;
            return Some((TokenType::ListStart, '('.to_string()))
        }

        if input.starts_with(')') {
            self.pos += 1;
            return Some((TokenType::ListEnd, ')'.to_string()))
        }

        let number_regexp = Regex::new(r"^-?\d+").unwrap();
        if let Some(cap) = number_regexp.captures(input) {
            self.pos += cap[0].len();
            return Some((TokenType::Number, cap[0].to_string()))
        }
        None
    }
}


fn parse_internal(tokenizer: &mut Iterator<Item=(TokenType, String)>) -> Result<MistType, String> {
    let mut tokenizer = tokenizer.peekable();
    if let Some(token) = tokenizer.next() {
        return match token {
            (TokenType::Number, token_string) => {
                Ok(Int(token_string.parse().unwrap()))
            }

            (TokenType::ListStart, _token_string) => {
                let mut list = vec![];
                loop {
                    let token_option = tokenizer.peek().cloned();
                    if let Some(element_token) = token_option {
                        match element_token {
                            (TokenType::ListEnd, _) => {
                                break;
                            }
                            _ => {
                                let parsed_element = parse_internal(&mut tokenizer)?;
                                list.push(parsed_element);
                            }
                        }
                    } else {
                        return Err("Unexpected end of input".to_string());
                    }
                }
                Ok(List(list))
            }

            (TokenType::ListEnd, token_string) => {
                Err("Unexpected end of list".to_string() + &token_string[..])
            }
        }
    }

    Err("Error".to_string())
}

pub fn parse(input: &str) -> Result<MistType, String> {
    let mut tokenizer = Tokenizer::new(input);
    return parse_internal(&mut tokenizer);
}


#[test]
fn test_parse_number() {
    assert_eq!(parse("0"), Ok(Int(0)));
    assert_eq!(parse("1"), Ok(Int(1)));
    assert_eq!(parse("42"), Ok(Int(42)));
    assert_eq!(parse("-42"), Ok(Int(-42)));
}

fn tokenize(input: &str) -> Vec<(TokenType, String)> {
    Tokenizer::new(input).collect()
}

#[test]
fn test_tokenizer() {
    assert_eq!(tokenize("42"), vec![(TokenType::Number, "42".to_string())]);
    assert_eq!(tokenize("("), vec![(TokenType::ListStart, "(".to_string())]);
    assert_eq!(tokenize(")"), vec![(TokenType::ListEnd, ")".to_string())]);
    assert_eq!(tokenize("( )"), vec![
        (TokenType::ListStart, "(".to_string()),
        (TokenType::ListEnd, ")".to_string())
    ]);
    assert_eq!(tokenize("(\n)"), vec![
        (TokenType::ListStart, "(".to_string()),
        (TokenType::ListEnd, ")".to_string())
    ]);
    assert_eq!(tokenize("(42)"), vec![
        (TokenType::ListStart, "(".to_string()),
        (TokenType::Number, "42".to_string()),
        (TokenType::ListEnd, ")".to_string())
    ]);
}

#[test]
fn test_list() {
    assert_eq!(parse("()"), Ok(List(vec![])));
    assert_eq!(parse("(42)"), Ok(List(vec![Int(42)])));
    assert_eq!(parse("(42 23)"), Ok(List(vec![Int(42), Int(23)])));
}
