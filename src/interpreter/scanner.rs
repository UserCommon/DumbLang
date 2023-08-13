
use crate::utils::throw_error;
use super::token::{Token, TokenType, KEYWORDS};
use TokenType::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source, tokens: vec![], current: 0, start: 0, line: 1 }
    }

    fn scan_tokens(&mut self) -> () {
        while !self.is_ended() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(Eof, "", self.line));
    }

    fn scan_token(&mut self) {
        // FIXME if-let ?
        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' => {
                ();
            },
            '(' => {
                self.pushToken(LeftParen);
            },

            ')' => {
                self.pushToken(RightParen);
            },

            '{' => {
                self.pushToken(LeftBrace);
            },

            '}' => {
                self.pushToken(LeftBrace);
            },

            ',' => {
                self.pushToken(Comma);
            },

            '.' => {
                self.pushToken(Dot);
            },

            '-' => {
                self.pushToken(Minus);
            },

            '+' => {
                self.pushToken(Plus);
            },

            '*' => {
                self.pushToken(Star);
            },

            ';' => {
                self.pushToken(Semicolon);
            },

            '!' => {
                if self.nmatch('=') {
                    self.pushToken(BangEqual);
                } else {
                    self.pushToken(Bang);
                }
            },

            '=' => {
                if self.nmatch('=') {
                    self.pushToken(EqualEqual);
                } else {
                    self.pushToken(Equal);
                }
            },

            '<' => {
                if self.nmatch('=') {
                    self.pushToken(LessEqual);
                } else {
                    self.pushToken(Less);
                }
            },

            '>' => {
                if self.nmatch('=') {
                    self.pushToken(GreaterEqual);
                } else {
                    self.pushToken(Greater);
                }
            } 

            '/' => {
                if self.nmatch('/') {
                    while self.peek() != '\n' && !self.is_ended() {
                        self.advance();
                    }
                } else {
                    self.pushToken(Slash)
                }
            },

            '\n' => {
                self.line += 1;
            },

            '"' => {
                self.collectString();
            },
            
            _ => {
                if c.is_numeric() {
                    self.collectNumber();
                } else if c.is_alphabetic() {
                    self.collectIdentifier();
                } else {
                    throw_error(self.line, Result::Err("Unexpected token".to_string()))
                }
            }
        };
        ()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn nmatch(&mut self, expected: char) -> bool {
        if self.is_ended() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn pushToken(&mut self, token_type: TokenType) {
        // TODO! refactoring may be required in future
        let text = &self.source[(self.start as usize)..(self.current as usize)];
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn collectString(&mut self) {
        while (self.peek() != '"') && (!self.is_ended()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_ended() {
            throw_error(self.line, Result::Err("unterminated string".to_string()));
        }
        self.advance();

        let value: String = (&self.source[self.start as usize - 1..self.current as usize - 1]).to_string();
        self.pushToken(Str(value));
    }

    fn collectNumber(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }
        let num = &self.source[(self.start) as usize .. self.current as usize].to_string();
        self.pushToken(Number(num.parse::<f32>().expect(&format!("cant unwrap number: {num}"))));
    }

    fn collectIdentifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start as usize..self.current as usize].to_string();
        let token: Option<&TokenType> = KEYWORDS.get(&text);

        match token {
            None => self.pushToken(Identifier),
            Some(x) => self.pushToken(x.clone() )
        };
    }


    fn peek(&self) -> char {
        self.source.chars().nth(self.current as usize).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth((self.current + 1) as usize).unwrap_or('\0')
    }
 

    fn is_ended(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        self.scan_tokens();
        &self.tokens
    }
}

#[cfg(test)]
mod interpreter_test {
    use crate::interpreter::token::Token;

    use super::Scanner;


    #[test]
    fn tokenize_file() {
        let file = r#"
            let some = "kwakky";
            let some_2 = "kwak ky ";
        "#;

        let mut scan = Scanner::new(file.into());
    }
}