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

lazy_static! {
    static ref COMMENT_REGEXP: Regex = Regex::new("^(\\s+|;.*?(\n|$))+").unwrap();
    static ref STR_REGEXP: Regex = Regex::new("^\".*?\"").unwrap();
    static ref SYMBOL_REGEXP: Regex = Regex::new(r"^[^\s\{\}()\[\]]+").unwrap();
    static ref NUMBER_REGEXP: Regex = Regex::new(r"^-?\d+").unwrap();
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
        // Skip white space and comments
        if let Some(cap) = COMMENT_REGEXP.captures(&self.input[self.pos..]) {
            self.pos += cap[0].len();
        }

        let input = &self.input[self.pos..];


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

        if let Some(cap) = STR_REGEXP.captures(input) {
            self.pos += cap[0].len();
            return Some(token(TokenType::Str, cap[0].to_string()))
        }

        if let Some(cap) = NUMBER_REGEXP.captures(input) {
            self.pos += cap[0].len();
            return Some(token(TokenType::Number, cap[0].to_string()))
        }

        if let Some(cap) = SYMBOL_REGEXP.captures(input) {
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
pub fn tokenize(input: &str) -> Vec<Token> {
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

#[test]
fn test_ignore_single_line_comment() {
    assert_eq!(tokenize("; comment"), vec![]);
    assert_eq!(tokenize("; comment\n"), vec![]);
    assert_eq!(tokenize("; comment\n 23"), vec![token(TokenType::Number, "23")]);
}

#[test]
fn test_negative_int() {
    assert_eq!(tokenize("-23"), vec![token(TokenType::Number, "-23")]);
}