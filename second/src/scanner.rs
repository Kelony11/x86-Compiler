#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),

    // arithmetic / parens
    Plus,
    Minus,        // <-- added
    Multiply,
    LeftParen,
    RightParen,

    // punctuation / separators
    Semicolon,
    Assign,
    Comma,
    LeftBrace,
    RightBrace,

    // comparisons
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
    EqualEqualTo,

    // keywords
    KeywordArgs, KeywordInt, KeywordIf, KeywordThen, KeywordElse,
    KeywordWhile, KeywordTrue, KeywordFalse, KeywordReturn,

    // end / misc
    EndOfFile,
    Invalid,
}

pub struct Scanner {
    position: usize,
    input_chars: Vec<char>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { position: 0, input_chars: source.chars().collect() }
    }

    fn current_char(&self) -> Option<char> {
        self.input_chars.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current_char();
        if ch.is_some() { self.position += 1; }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() { self.advance(); } else { break; }
        }
    }

    fn collect_while<F: Fn(char) -> bool>(&mut self, first: char, cond: F) -> String {
        let mut text = String::from(first);
        while let Some(next) = self.current_char() {
            if cond(next) { text.push(self.advance().unwrap()); } else { break; }
        }
        text
    }

    // Map an identifier string into a keyword token when applicable
    fn keyword_or_ident(s: String) -> Token {
        match s.as_str() {
            "args"   => Token::KeywordArgs,
            "int"    => Token::KeywordInt,
            "if"     => Token::KeywordIf,
            "then"   => Token::KeywordThen,
            "else"   => Token::KeywordElse,
            "while"  => Token::KeywordWhile,
            "true"   => Token::KeywordTrue,
            "false"  => Token::KeywordFalse,
            "return" => Token::KeywordReturn,
            _        => Token::Identifier(s),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let ch = self.advance()?; // EOF → None

        // Multi-char operators first (using lookahead at current_char())
        let tok = match ch {
            // <= or <
            '<' => {
                if matches!(self.current_char(), Some('=')) {
                    self.advance();
                    Token::LessThanEqualTo
                } else {
                    Token::LessThan
                }
            }

            // >= or >
            '>' => {
                if matches!(self.current_char(), Some('=')) {
                    self.advance();
                    Token::GreaterThanEqualTo
                } else {
                    Token::GreaterThan
                }
            }

            // == or =
            '=' => {
                if matches!(self.current_char(), Some('=')) {
                    self.advance();
                    Token::EqualEqualTo
                } else {
                    Token::Assign
                }
            }

            // single-char punctuation
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            // arithmetic
            '+' => Token::Plus,
            '-' => Token::Minus,        // <-- added
            '*' => Token::Multiply,

            // number literal (unsigned)
            d if d.is_ascii_digit() => {
                let digits = self.collect_while(d, |c| c.is_ascii_digit());
                Token::Number(digits)
            }

            // identifier / keyword: [A-Za-z_][A-Za-z0-9_]*
            a if a.is_ascii_alphabetic() || a == '_' => {
                let ident = self.collect_while(a, |c| c.is_ascii_alphanumeric() || c == '_');
                Scanner::keyword_or_ident(ident)
            }

            // unknown char
            _ => Token::Invalid,
        };

        Some(tok)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens.push(Token::EndOfFile);
        tokens
    }
}

pub fn scan_source(source: &str) -> Vec<Token> {
    let mut lexer = Scanner::new(source.to_string());
    lexer.tokenize()
}
