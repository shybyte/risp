use std::collections::HashMap;
use types::*;
use types::RispType::*;
use tokenize::*;

fn parse_internal(tokenizer: &mut Iterator<Item=Token>) -> Result<RispType, RispError> {
    let mut tokenizer = tokenizer.peekable();
    if let Some(token) = tokenizer.next() {
        return match token {
            (TokenType::Number, token_string) => {
                Ok(Int(token_string.parse().unwrap()))
            }

            (TokenType::Symbol, token_string) => {
                match &token_string[..] {
                    "true" => Ok(Bool(true)),
                    "false" => Ok(Bool(false)),
                    _ => Ok(symbol(token_string))
                }
            }

            (TokenType::Keyword, token_string) => {
                Ok(keyword(&token_string[1..]))
            }

            (TokenType::Str, token_string) => {
                Ok(string(&token_string[1..(token_string.len() - 1)]))
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
                        return error_result("Unexpected end of list");
                    }
                }
                Ok(List(list))
            }

            (TokenType::ListEnd, _token_string) => {
                error_result("Unexpected end of list")
            }

            (TokenType::VectorStart, _token_string) => {
                let mut vector = vec![];
                loop {
                    let token_option = tokenizer.peek().cloned();
                    if let Some(element_token) = token_option {
                        match element_token {
                            (TokenType::VectorEnd, _) => {
                                break;
                            }
                            _ => {
                                let parsed_element = parse_internal(&mut tokenizer)?;
                                vector.push(parsed_element);
                            }
                        }
                    } else {
                        return error_result("Vector should end with ] but just ends");
                    }
                }
                Ok(Vector(vector))
            }

            (TokenType::VectorEnd, _token_string) => {
                error_result("Unexpected ]")
            }

            (TokenType::HashMapStart, _token_string) => {
                let mut map = HashMap::<String, RispType>::new();
                loop {
                    let token_option = tokenizer.peek().cloned();
                    if let Some(element_token) = token_option {
                        match element_token {
                            (TokenType::HashMapEnd, _) => {
                                break;
                            }

                            (TokenType::Keyword, keyword) => {
                                tokenizer.next();
                                let parsed_element = parse_internal(&mut tokenizer)?;
                                map.insert((&keyword[1..]).to_string(), parsed_element);
                            }

                            (_, token_string) => {
                                return error_result(format!("Expected keyword but got {:?}", token_string));
                            }
                        }
                    } else {
                        return error_result("HashMap should end with } but just ends");
                    }
                }
                Ok(Map(map))
            }

            (TokenType::HashMapEnd, _token_string) => {
                error_result("Unexpected }")
            }
        }
    }

    error_result("Error")
}

pub fn parse(input: &str) -> Result<RispType, RispError> {
    let mut tokenizer = Tokenizer::new(input);
    parse_internal(&mut tokenizer)
}


/* ------------------------------ Tests ----------------------------------------------- */

#[test]
fn test_parse_number() {
    assert_eq!(parse("0"), Ok(Int(0)));
    assert_eq!(parse("1"), Ok(Int(1)));
    assert_eq!(parse("42"), Ok(Int(42)));
    assert_eq!(parse("-42"), Ok(Int(-42)));
}


#[test]
fn test_list() {
    assert_eq!(parse("()"), Ok(List(vec![])));
    assert_eq!(parse("(42)"), Ok(List(vec![Int(42)])));
    assert_eq!(parse("(42 23)"), Ok(List(vec![Int(42), Int(23)])));
    assert_eq!(parse("(42 (23))"), Ok(List(vec![Int(42), List(vec![Int(23)])])));
}

#[test]
fn test_parse_symbols() {
    assert_eq!(parse("symbol"), Ok(Symbol("symbol".to_string())));
    assert_eq!(parse("(+ 1 2)"), Ok(List(vec![Symbol("+".to_string()), Int(1), Int(2)])));
}

#[test]
fn test_parse_vector() {
    assert_eq!(parse("[]"), Ok(Vector(vec![])));
    assert_eq!(parse("[42]"), Ok(Vector(vec![Int(42)])));
    assert_eq!(parse("[42 23]"), Ok(Vector(vec![Int(42), Int(23)])));
    assert_eq!(parse("[42 [23]]"), Ok(Vector(vec![Int(42), Vector(vec![Int(23)])])));
}

#[test]
fn test_parse_vector_errors() {
    assert_eq!(parse("["), error_result("Vector should end with ] but just ends"));
    assert_eq!(parse("]"), error_result("Unexpected ]"));
    assert_eq!(parse("(]"), error_result("Unexpected ]"));
}


#[test]
fn test_keyword() {
    assert_eq!(parse(":key"), Ok(keyword("key")));
}


#[test]
fn test_hash_map_empty() {
    assert_eq!(parse("{}"), Ok(Map(HashMap::new())));
}

#[test]
fn test_hash_map_with_1_key() {
    assert_eq!(parse("{:key 123}"), Ok(map(vec![("key", Int(123))])));
}

#[test]
fn test_hash_map_with_2_keys() {
    let expected_map = map(vec![
        ("key1", Int(1)),
        ("key2", Int(2))
    ]);
    assert_eq!(parse("{:key1 1 :key2 2}"), Ok(expected_map));
}

#[test]
fn test_map_with_var() {
    assert_eq!(parse("{:key1 var}"), Ok(map(vec![
        ("key1", symbol("var"))
    ])));
}


#[test]
fn test_hash_map_errors() {
    assert_eq!(parse("{"), error_result("HashMap should end with } but just ends"));
    assert_eq!(parse("}"), error_result("Unexpected }"));
    assert_eq!(parse("{123}"), error_result("Expected keyword but got \"123\""));
}

#[test]
fn test_str() {
    assert_eq!(parse("\"string\""), Ok(string("string")));
}


#[test]
fn test_bool() {
    assert_eq!(parse("true"), Ok(Bool(true)));
    assert_eq!(parse("false"), Ok(Bool(false)));
}
