use crate::scanner::Token;
use crate::ast::ASTNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Parser for the expression language.
/// Implements a recursive descent parser based on the grammar:
///
/// EXPR     → TERM EXPR'
/// EXPR'    → + TERM EXPR' | ε
/// TERM     → FACTOR TERM'
/// TERM'    → * FACTOR TERM' | ε
/// FACTOR   → IDENTIFIER | NUMBER | ( EXPR )
///
/// This parser constructs an Abstract Syntax Tree (AST)
/// where operators (+, *) become internal nodes and
/// identifiers/numbers become leaf nodes.

pub struct Parser {
    token_stream: Vec<Token>,
    current_pos: usize,
}

impl Parser {
    /// Create a new parser from a token list.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_stream: tokens,
            current_pos: 0,
        }
    }

    /// Peek at the current token in the stream.
    fn peek(&self) -> &Token {
        self.token_stream
            .get(self.current_pos)
            .unwrap_or(&Token::EndOfFile)
    }

    /// Move to the next token in the stream.
    fn consume(&mut self) {
        if self.current_pos < self.token_stream.len() {
            self.current_pos += 1;
        }
    }

    // ============================================================
    // EXPR → TERM EXPR'
    // ============================================================
    /// Parse an expression (handles '+' at the top level).
    pub fn parse_expression(&mut self) -> Rc<RefCell<ASTNode>> {
        let term_node = self.parse_term();
        self.parse_expression_tail(term_node)
    }

    // ============================================================
    // EXPR' → + TERM EXPR' | ε
    // ============================================================
    /// Continue parsing if a '+' is found after a term.
    fn parse_expression_tail(&mut self, accumulated: Rc<RefCell<ASTNode>>) -> Rc<RefCell<ASTNode>> {
        match self.peek() {
            Token::Plus => {
                // consume '+'
                self.consume();
                let next_term = self.parse_term();
                // combine left and right with '+'
                let combined_expr = Rc::new(RefCell::new(ASTNode::Add(accumulated, next_term)));
                // continue checking for more '+'
                self.parse_expression_tail(combined_expr)
            }
            _ => accumulated, // no '+' → epsilon transition
        }
    }

    // ============================================================
    // TERM → FACTOR TERM'
    // ============================================================
    /// Parse a term (handles '*' operations).
    fn parse_term(&mut self) -> Rc<RefCell<ASTNode>> {
        let factor_node = self.parse_factor();
        self.parse_term_tail(factor_node)
    }

    // ============================================================
    // TERM' → * FACTOR TERM' | ε
    // ============================================================
    /// Continue parsing if a '*' follows a factor.
    fn parse_term_tail(&mut self, accumulated: Rc<RefCell<ASTNode>>) -> Rc<RefCell<ASTNode>> {
        match self.peek() {
            Token::Multiply => {
                // consume '*'
                self.consume();
                let next_factor = self.parse_factor();
                // combine with multiplication
                let combined_term = Rc::new(RefCell::new(ASTNode::Multiply(accumulated, next_factor)));
                // recursively continue
                self.parse_term_tail(combined_term)
            }
            _ => accumulated, // no '*' → epsilon transition
        }
    }

    // ============================================================
    // FACTOR → IDENTIFIER | NUMBER | ( EXPR )
    // ============================================================
    /// Parse a factor (base unit: identifier, number, or parenthesized expression).
    fn parse_factor(&mut self) -> Rc<RefCell<ASTNode>> {
        match self.peek() {
            // If it's a number → create a numeric node
            Token::Number(num_value) => {
                let number_node = Rc::new(RefCell::new(ASTNode::Number(num_value.clone())));
                self.consume();
                number_node
            }

            // If it's an identifier → create a variable node
            Token::Identifier(var_name) => {
                let ident_node = Rc::new(RefCell::new(ASTNode::Identifier(var_name.clone())));
                self.consume();
                ident_node
            }

            // If it's '(' → parse a full expression inside the parentheses
            Token::LeftParen => {
                self.consume(); // consume '('
                let inner_expr = self.parse_expression();

                // expect closing ')'
                if let Token::RightParen = self.peek() {
                    self.consume(); // consume ')'
                    inner_expr
                } else {
                    panic!("Syntax error: expected ')' after expression");
                }
            }

            // Any other token → invalid factor
            unexpected_token => {
                panic!("Unexpected token in factor: {:?}", unexpected_token);
            }
        }
    }
}
