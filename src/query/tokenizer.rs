#[derive(Debug, Clone)]
pub enum Token {
    Field(String),
    Operator(String),
    StringLiteral(String),
    NumberLiteral(String),
    LParen,
    RParen,
    Function(String),
    And,
    Or,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.pos >= self.input.len() {
                break;
            }
            let ch = self.input[self.pos];
            if ch.is_ascii_digit() {
                tokens.push(self.read_number());
            } else if ch == '\'' || ch == '"' {
                tokens.push(self.read_string());
            } else if ch == '(' {
                tokens.push(Token::LParen);
                self.pos += 1;
            } else if ch == ')' {
                tokens.push(Token::RParen);
                self.pos += 1;
            } else if ch.is_alphabetic() || ch == '_' {
                tokens.push(self.read_identifier());
            } else if ch == '=' || ch == '!' || ch == '>' || ch == '<' {
                tokens.push(self.read_operator());
            } else if ch == ',' {
                self.pos += 1;
            } else {
                self.pos += 1;
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == '.')
        {
            self.pos += 1;
        }
        Token::NumberLiteral(self.input[start..self.pos].iter().collect())
    }

    fn read_string(&mut self) -> Token {
        let quote = self.input[self.pos];
        self.pos += 1;
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != quote {
            self.pos += 1;
        }
        let val = self.input[start..self.pos].iter().collect();
        self.pos += 1;
        Token::StringLiteral(val)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_alphanumeric()
                || self.input[self.pos] == '_'
                || self.input[self.pos] == '.')
        {
            self.pos += 1;
        }
        let ident: String = self.input[start..self.pos].iter().collect();

        if ident == "has" {
            return Token::Function(ident);
        }

        if ident == "and" {
            return Token::And;
        }
        if ident == "or" {
            return Token::Or;
        }

        Token::Field(ident)
    }

    fn read_operator(&mut self) -> Token {
        let start = self.pos;
        let ch = self.input[self.pos];
        self.pos += 1;

        if self.pos < self.input.len() {
            let next = self.input[self.pos];
            if (ch == '=' && next == '=')
                || (ch == '!' && next == '=')
                || (ch == '>' && next == '=')
                || (ch == '<' && next == '=')
                || (ch == '=' && next == '~')
            {
                self.pos += 1;
                return Token::Operator(self.input[start..self.pos].iter().collect());
            }
        }

        Token::Operator(ch.to_string())
    }
}
