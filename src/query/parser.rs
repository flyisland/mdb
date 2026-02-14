use super::tokenizer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum AstNode {
    Binary {
        left: Box<AstNode>,
        op: String,
        right: Box<AstNode>,
    },
    Field(String),
    StringLiteral(String),
    NumberLiteral(String),
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Grouping(Box<AstNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_or()
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        self.pos += 1;
        token
    }

    fn parse_or(&mut self) -> AstNode {
        let mut left = self.parse_and();

        while matches!(self.current(), Token::Or) {
            self.advance();
            let right = self.parse_and();
            left = AstNode::Binary {
                left: Box::new(left),
                op: "OR".to_string(),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_and(&mut self) -> AstNode {
        let mut left = self.parse_comparison();

        while matches!(self.current(), Token::And) {
            self.advance();
            let right = self.parse_comparison();
            left = AstNode::Binary {
                left: Box::new(left),
                op: "AND".to_string(),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_comparison(&mut self) -> AstNode {
        let left = self.parse_primary();

        if let Token::Operator(op) = self.current().clone() {
            if ["==", "!=", ">", "<", ">=", "<=", "=~"].contains(&op.as_str()) {
                self.advance();
                let right = self.parse_primary();
                return AstNode::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
        }

        left
    }

    fn parse_primary(&mut self) -> AstNode {
        match self.current().clone() {
            Token::LParen => {
                self.advance();
                let expr = self.parse_or();
                self.advance();
                AstNode::Grouping(Box::new(expr))
            }
            Token::Function(name) => {
                self.advance();
                if let Token::LParen = self.current().clone() {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.current(), Token::RParen) {
                        args.push(self.parse_primary());
                        if matches!(self.current(), Token::RParen) {
                            break;
                        }
                        self.advance();
                    }
                    self.advance();
                    return AstNode::FunctionCall { name, args };
                }
                AstNode::FunctionCall { name, args: vec![] }
            }
            Token::Field(name) => {
                self.advance();
                AstNode::Field(name)
            }
            Token::StringLiteral(val) => {
                self.advance();
                AstNode::StringLiteral(val)
            }
            Token::NumberLiteral(val) => {
                self.advance();
                AstNode::NumberLiteral(val)
            }
            _ => AstNode::StringLiteral(String::new()),
        }
    }
}

pub fn parse(query: &str) -> AstNode {
    let mut lexer = Lexer::new(query);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse()
}
