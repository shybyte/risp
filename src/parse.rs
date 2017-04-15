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
                Ok(symbol(token_string))
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
