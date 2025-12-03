use crate::scanner::Token;
use crate::ast::{ASTNode, BooleanExpression, Statement, Program};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser {
    token_stream: Vec<Token>,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_stream: tokens,
            current_pos: 0,
        }
    }

    fn peek(&self) -> &Token {
        self.token_stream
            .get(self.current_pos)
            .unwrap_or(&Token::EndOfFile)
    }

    fn consume(&mut self) {
        if self.current_pos < self.token_stream.len() {
            self.current_pos += 1;
        }
    }

    fn expect(&mut self, expected: &Token) {
        if self.peek() == expected {
            self.consume();
        } else {
            panic!("Syntax Error: expected {:?}, found {:?}", expected, self.peek());
        }
    }

    // ============================================================
    // TOP-LEVEL: parse_program()
    // ============================================================
    pub fn parse_program(&mut self) -> Program {
        let arg_declarations = self.parse_argdecl();
        let var_declarations = self.parse_typedecl();
        let statements = self.parse_statements();
        let return_ident = self.parse_return();

        Program {
            arg_declarations,
            var_declarations,
            statements,
            return_ident,
        }
    }

    // ============================================================
    // ARGDECL → args IDENTIFIER ARGDECLTAIL
    // ARGDECLTAIL → ; | IDENTIFIER ARGDECLTAIL
    // ============================================================
    fn parse_argdecl(&mut self) -> Vec<String> {
        let mut args = Vec::new();

        match self.peek() {
            Token::KeywordArgs => {
                self.consume(); // consume 'args'

                if let Token::Identifier(name) = self.peek().clone() {
                    args.push(name);
                    self.consume();
                } else {
                    panic!("Expected identifier after 'args'");
                }

                loop {
                    match self.peek() {
                        Token::Semicolon => {
                            self.consume();
                            break;
                        }
                        Token::Identifier(name) => {
                            args.push(name.clone());
                            self.consume();
                        }
                        _ => panic!("Unexpected token in argument list: {:?}", self.peek()),
                    }
                }
            }
            _ => panic!("Program must start with 'args'"),
        };

        args
    }

    // ============================================================
    // TYPEDECL → int IDENTIFIER TYPEDECLTAIL
    // TYPEDECLTAIL → ; | , IDENTIFIER TYPEDECLTAIL
    // ============================================================
    fn parse_typedecl(&mut self) -> Vec<String> {
        let mut vars = Vec::new();

        self.expect(&Token::KeywordInt);

        // first identifier
        if let Token::Identifier(name) = self.peek().clone() {
            vars.push(name);
            self.consume();
        } else {
            panic!("Expected identifier after 'int'");
        }

        loop {
            match self.peek() {
                Token::Comma => {
                    self.consume();
                    if let Token::Identifier(name) = self.peek().clone() {
                        vars.push(name);
                        self.consume();
                    } else {
                        panic!("Expected identifier after ','");
                    }
                }
                Token::Semicolon => {
                    self.consume();
                    break;
                }
                _ => panic!("Unexpected token in type declaration: {:?}", self.peek()),
            }
        }

        vars
    }

    // ============================================================
    // STMTS → STMT STMTS | ε
    // ============================================================
    fn parse_statements(&mut self) -> Vec<Statement> {
        let mut stmts = Vec::new();

        loop {
            match self.peek() {
                Token::Identifier(_) |
                Token::KeywordIf |
                Token::KeywordWhile => {
                    stmts.push(self.parse_statement());
                }

                // ε transition — statement list ends before return
                _ => break,
            }
        }

        stmts
    }

    // ============================================================
    // STMT → ASSIGN | IFTHENELSE | WHILE
    // ============================================================
    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Token::Identifier(_) => self.parse_assign(),

            Token::KeywordIf => self.parse_if_statement(),

            Token::KeywordWhile => self.parse_while_statement(),

            _ => panic!("Invalid statement start: {:?}", self.peek()),
        }
    }

    // ============================================================
    // ASSIGN → IDENTIFIER = EXPR ;
    // ============================================================
    fn parse_assign(&mut self) -> Statement {
        // identifier
        let name = if let Token::Identifier(name) = self.peek().clone() {
            name
        } else {
            panic!("Expected identifier in assignment");
        };
        self.consume();

        // =
        self.expect(&Token::Assign);

        // expression
        let expr = self.parse_expression();

        // ;
        self.expect(&Token::Semicolon);

        Statement::Assign { name, expression: expr }
    }

    // ============================================================
    // IFTHENELSE → if BOOL then { STMTS } else { STMTS }
    // ============================================================
    fn parse_if_statement(&mut self) -> Statement {
        self.expect(&Token::KeywordIf);
        let cond = self.parse_boolean();
        self.expect(&Token::KeywordThen);

        self.expect(&Token::LeftBrace);
        let then_part = self.parse_statements();
        self.expect(&Token::RightBrace);

        self.expect(&Token::KeywordElse);
        self.expect(&Token::LeftBrace);
        let else_part = self.parse_statements();
        self.expect(&Token::RightBrace);

        Statement::If {
            condition: cond,
            then_statements: then_part,
            else_statements: else_part,
        }
    }

    // ============================================================
    // WHILE → while BOOL then { STMTS }
    // ============================================================
    fn parse_while_statement(&mut self) -> Statement {
        self.expect(&Token::KeywordWhile);
        let cond = self.parse_boolean();
        self.expect(&Token::KeywordThen);

        self.expect(&Token::LeftBrace);
        let body = self.parse_statements();
        self.expect(&Token::RightBrace);

        Statement::While {
            condition: cond,
            body,
        }
    }

    // ============================================================
    // RET → return IDENTIFIER ;
    // ============================================================
    fn parse_return(&mut self) -> String {
        self.expect(&Token::KeywordReturn);

        let ident = if let Token::Identifier(name) = self.peek().clone() {
            name
        } else {
            panic!("Expected identifier after 'return'");
        };
        self.consume();

        self.expect(&Token::Semicolon);

        ident
    }

    // ============================================================
    // BOOLEAN EXPRESSIONS
    // ============================================================
    fn parse_boolean(&mut self) -> BooleanExpression {
        match self.peek() {
            Token::KeywordTrue => {
                self.consume();
                BooleanExpression::True
            }
            Token::KeywordFalse => {
                self.consume();
                BooleanExpression::False
            }
            _ => {
                // BOOL -> EXPR < EXPR | ... (needs two EXPRs)
                let left = self.parse_expression();

                match self.peek() {
                    Token::LessThan => {
                        self.consume();
                        let right = self.parse_expression();
                        BooleanExpression::CompareLessThan(left, right)
                    }
                    Token::LessThanEqualTo => {
                        self.consume();
                        let right = self.parse_expression();
                        BooleanExpression::CompareLessThanEqualTo(left, right)
                    }
                    Token::GreaterThan => {
                        self.consume();
                        let right = self.parse_expression();
                        BooleanExpression::CompareGreaterThan(left, right)
                    }
                    Token::GreaterThanEqualTo => {
                        self.consume();
                        let right = self.parse_expression();
                        BooleanExpression::CompareGreaterThanEqualTo(left, right)
                    }
                    Token::EqualEqualTo => {
                        self.consume();
                        let right = self.parse_expression();
                        BooleanExpression::CompareEqualTo(left, right)
                    }
                    _ => panic!("Invalid boolean operator: {:?}", self.peek()),
                }
            }
        }
    }

    // ============================================================
    // ====== EXPRESSION PARSER (YOUR ORIGINAL, WITH SUB ADDED) ===
    // EXPR → EXPR + TERM | EXPR - TERM | TERM
    // ============================================================
    pub fn parse_expression(&mut self) -> Rc<RefCell<ASTNode>> {
        let term_node = self.parse_term();
        self.parse_expression_tail(term_node)
    }

    fn parse_expression_tail(&mut self, accumulated: Rc<RefCell<ASTNode>>) -> Rc<RefCell<ASTNode>> {
        match self.peek() {
            Token::Plus => {
                self.consume();
                let next_term = self.parse_term();
                let combined = Rc::new(RefCell::new(ASTNode::Add(accumulated, next_term)));
                self.parse_expression_tail(combined)
            }
            Token::Minus => {
                self.consume();
                let next_term = self.parse_term();
                let combined = Rc::new(RefCell::new(ASTNode::Sub(accumulated, next_term)));
                self.parse_expression_tail(combined)
            }
            _ => accumulated,
        }
    }

    // TERM → TERM * FACTOR | FACTOR
    fn parse_term(&mut self) -> Rc<RefCell<ASTNode>> {
        let factor_node = self.parse_factor();
        self.parse_term_tail(factor_node)
    }

    fn parse_term_tail(&mut self, accumulated: Rc<RefCell<ASTNode>>) -> Rc<RefCell<ASTNode>> {
        match self.peek() {
            Token::Multiply => {
                self.consume();
                let next_factor = self.parse_factor();
                let combined =
                    Rc::new(RefCell::new(ASTNode::Multiply(accumulated, next_factor)));
                self.parse_term_tail(combined)
            }
            _ => accumulated,
        }
    }

    fn parse_factor(&mut self) -> Rc<RefCell<ASTNode>> {
        match self.peek() {
            Token::Number(v) => {
                let node = Rc::new(RefCell::new(ASTNode::Number(v.clone())));
                self.consume();
                node
            }
            Token::Identifier(name) => {
                let node = Rc::new(RefCell::new(ASTNode::Identifier(name.clone())));
                self.consume();
                node
            }
            Token::LeftParen => {
                self.consume();
                let inner = self.parse_expression();
                if let Token::RightParen = self.peek() {
                    self.consume();
                } else {
                    panic!("Expected ')'");
                }
                inner
            }
            other => panic!("Unexpected token in factor: {:?}", other),
        }
    }
}
