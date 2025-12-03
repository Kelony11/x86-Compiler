#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    Plus,      
    Multiply, 
    LeftParen,  
    RightParen,  
    Invalid,     
    EndOfFile,   
}



pub struct Scanner {
    position: usize,        // current reading index
    input_chars: Vec<char>, // input stored as character vector
}

impl Scanner {
    /// Constructs a new Scanner from a raw input string
    pub fn new(source: String) -> Self {
        Scanner {
            position: 0,
            input_chars: source.chars().collect(),
        }
    }

    /// Returns the character currently under the reading index
    fn current_char(&self) -> Option<char> {
        self.input_chars.get(self.position).copied()
    }

    /// Advances the scanner by one character and returns it
    fn advance(&mut self) -> Option<char> {
        let current = self.current_char();
        if current.is_some() {
            self.position += 1;
        }
        current
    }

    /// Skips over whitespace characters (spaces, tabs, newlines)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Helper function: keeps collecting characters while the
    /// given predicate returns true (e.g. `is_digit` or `is_alpha`)
    fn collect_while<F: Fn(char) -> bool>(&mut self, first: char, cond: F) -> String {
        let mut text = String::from(first);
        while let Some(next) = self.current_char() {
            if cond(next) {
                text.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        text
    }

    /// Reads and returns the next token from the input stream.
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace(); // ignore spaces

        let ch = self.advance()?; // move to next char, or None if EOF

        let token = match ch {
            '+' => Token::Plus,
            '*' => Token::Multiply,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            // Begin numeric literal
            d if d.is_ascii_digit() => {
                let digits = self.collect_while(d, |c| c.is_ascii_digit());
                Token::Number(digits)
            }

            // Begin identifier
            a if a.is_alphabetic() => {
                let ident = self.collect_while(a, |c| c.is_alphabetic());
                Token::Identifier(ident)
            }

            // Anything else is not valid in this grammar
            _ => Token::Invalid,
        };

        Some(token)
    }

    /// Produces a complete list of tokens from the input string.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Continue reading tokens until exhaustion
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }

        // Append EOF marker for parser convenience
        tokens.push(Token::EndOfFile);
        tokens
    }
}


pub fn scan_source(source: &str) -> Vec<Token> {
    let mut lexer = Scanner::new(source.to_string());
    lexer.tokenize()
}
