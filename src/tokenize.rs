use regex::Regex;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Number,
    ListStart,
    ListEnd,
    VectorStart,
    VectorEnd,
    Symbol
}

pub type Token = (TokenType, String);

pub struct Tokenizer {
    input: String,
    pos: usize
}

impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer { input: input.to_string(), pos: 0 }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let input = &self.input[self.pos..];

        // Skip white space
        let input_trimmed = input.trim_left();
        self.pos += input.len() - input_trimmed.len();
        let input = input_trimmed;

        if input.starts_with('(') {
            self.pos += 1;
            return Some(token(TokenType::ListStart, "("))
        }

        if input.starts_with(')') {
            self.pos += 1;
            return Some(token(TokenType::ListEnd, ")"))
        }

        if input.starts_with('[') {
            self.pos += 1;
            return Some(token(TokenType::VectorStart, "["))
        }

        if input.starts_with(']') {
            self.pos += 1;
            return Some(token(TokenType::VectorEnd, "]"));
        }

        let number_regexp = Regex::new(r"^-?\d+").unwrap();
        if let Some(cap) = number_regexp.captures(input) {
            self.pos += cap[0].len();
            return Some(token(TokenType::Number, cap[0].to_string()))
        }

        let symbol_regexp = Regex::new(r"^\S+").unwrap();
        if let Some(cap) = symbol_regexp.captures(input) {
            self.pos += cap[0].len();
            return Some((TokenType::Symbol, cap[0].to_string()))
        }


        None
    }
}

fn token<S: Into<String>>(token_type: TokenType, s: S) -> Token {
    (token_type, s.into())
}



/* ------------------------------ Tests ----------------------------------------------- */

#[allow(dead_code)]
fn tokenize(input: &str) -> Vec<Token> {
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
fn test_tokenizer_symbol() {
    assert_eq!(tokenize("symbol"), vec![(TokenType::Symbol, "symbol".to_string())]);
    assert_eq!(tokenize("(+ 42)"), vec![
        (TokenType::ListStart, "(".to_string()),
        (TokenType::Symbol, "+".to_string()),
        (TokenType::Number, "42".to_string()),
        (TokenType::ListEnd, ")".to_string())
    ]);
}

#[test]
fn test_tokenizer_vector() {
    assert_eq!(tokenize("[]"), vec![
        token(TokenType::VectorStart, "["),
        token(TokenType::VectorEnd, "]")
    ]);
    assert_eq!(tokenize("[23 42]"), vec![
        token(TokenType::VectorStart, "["),
        token(TokenType::Number, "23"),
        token(TokenType::Number, "42"),
        token(TokenType::VectorEnd, "]")
    ]);
}