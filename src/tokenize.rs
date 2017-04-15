use regex::Regex;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Number,
    ListStart,
    ListEnd,
    VectorStart,
    VectorEnd,
    HashMapStart,
    HashMapEnd,
    Symbol,
    Keyword,
    Str
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

        if input.starts_with('{') {
            self.pos += 1;
            return Some(token(TokenType::HashMapStart, "{"))
        }

        if input.starts_with('}') {
            self.pos += 1;
            return Some(token(TokenType::HashMapEnd, "}"));
        }

        let str_regexp = Regex::new("^\".*?\"").unwrap();
        if let Some(cap) = str_regexp.captures(input) {
            self.pos += cap[0].len();
            return Some(token(TokenType::Str, cap[0].to_string()))
        }

        let number_regexp = Regex::new(r"^-?\d+").unwrap();
        if let Some(cap) = number_regexp.captures(input) {
            self.pos += cap[0].len();
            return Some(token(TokenType::Number, cap[0].to_string()))
        }

        let symbol_regexp = Regex::new(r"^[^\s\{\}()\[\]]+").unwrap();
        if let Some(cap) = symbol_regexp.captures(input) {
            self.pos += cap[0].len();
            let cap_string = cap[0].to_string();
            if cap_string.starts_with(':') {
                return Some((TokenType::Keyword, cap_string))
            } else {
                return Some((TokenType::Symbol, cap_string))
            }
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

#[test]
fn test_empty_map() {
    assert_eq!(tokenize("{}"), vec![
        token(TokenType::HashMapStart, "{"),
        token(TokenType::HashMapEnd, "}")
    ]);
}

#[test]
fn test_map() {
    assert_eq!(tokenize("{:keyword 123}"), vec![
        token(TokenType::HashMapStart, "{"),
        token(TokenType::Keyword, ":keyword"),
        token(TokenType::Number, "123"),
        token(TokenType::HashMapEnd, "}")
    ]);
}

#[test]
fn test_map_ending_with_symbol_as_value() {
    assert_eq!(tokenize("{:keyword var}"), vec![
        token(TokenType::HashMapStart, "{"),
        token(TokenType::Keyword, ":keyword"),
        token(TokenType::Symbol, "var"),
        token(TokenType::HashMapEnd, "}")
    ]);
}

#[test]
fn test_list_ending_with_symbol() {
    assert_eq!(tokenize("(symbol)"), vec![
        token(TokenType::ListStart, "("),
        token(TokenType::Symbol, "symbol"),
        token(TokenType::ListEnd, ")")
    ]);
}

#[test]
fn test_vector_ending_with_symbol() {
    assert_eq!(tokenize("[symbol]"), vec![
        token(TokenType::VectorStart, "["),
        token(TokenType::Symbol, "symbol"),
        token(TokenType::VectorEnd, "]")
    ]);
}

#[test]
fn test_string() {
    assert_eq!(tokenize("\"string\" \"\""), vec![
        token(TokenType::Str, "\"string\""),
        token(TokenType::Str, "\"\""),
    ]);
}