
use crate::utils::throw_error;
use super::token::{Token, TokenType};
use TokenType::*;

pub struct Scanner<T: std::fmt::Display> {
    source: String,
    tokens: Vec<Token<T>>,
    start: u32,
    current: u32,
    line: u32,
}

impl<T: std::fmt::Display> Scanner<T> {
    pub fn new(source: String) -> Self {
        Self { source, tokens: vec![], current: 0, start: 0, line: 1 }
    }

    fn scan_tokens(&mut self) -> Vec<Token<T>> {
        let prepared_src = self.source.split(" ");
        for src_token in prepared_src {
            self.scan_token(src_token);
        }
        self.tokens.push(Token::new(Eof, "", None, self.line));
        &mut self.tokens
    
    }

    fn scan_token(&mut self, token: &str) {
        // FIXME if-let ?
        if !token.is_empty() && token.chars().nth(0).unwrap() == '\n' {
            self.line += 1;
            self.scan_token(&token[1..]);
        }
        match token {
            " " | "\r" | "\t" => {
                ();
            },
            "(" => {
                self.pushToken(LeftParen);
            },

            ")" => {
                self.pushToken(RightParen);
            },

            "{" => {
                self.pushToken(LeftBrace);
            },

            "}" => {
                self.pushToken(LeftBrace);
            },

            "," => {
                self.pushToken(Comma);
            },

            "." => {
                self.pushToken(Dot);
            },

            "-" => {
                self.pushToken(Minus);
            },

            "+" => {
                self.pushToken(Plus);
            },

            "*" => {
                self.pushToken(Star);
            },

            "/" => {
                self.pushToken(Slash);
            },

            ";" => {
                self.pushToken(Semicolon);
            },

            "!" => {
                self.pushToken(Bang);
            },

            "<" => {
                self.pushToken(Less);
            },

            ">" => {
                self.pushToken(Greater);
            }

            // 2
            "!=" => {
                self.pushToken(BangEqual);
            },

            "==" => {
                self.pushToken(Equal);
            },

            "<=" => {
                self.pushToken(LessEqual);
            },

            ">=" => {
                self.pushToken(GreaterEqual);
            },

            "//" => {
                // FIXME idunno how to implement comments without mutating src...
            },

            r#"""# => {
                // FIXME there is trouble due splitting
                // It's causes situatin where "123 123" 
                // Lexs as ("123)  (123") like independent tokens
                // but it's should wait until " at end

                // man...
                // welp, there is solution...
                // Iterating over the source twice, making "" literals one token
                self.pushToken(Str("123".to_string()));
            }
            
            
            _ => {
                throw_error(self.line, Result::Err("Unexpected token".to_string()))
            }
        };
        ()
    }

    fn pushToken(&mut self, token_type: TokenType) {
        // TODO! refactoring may be required in future
        self.tokens.push(Token::new(token_type, "x", None, self.line));
    }

    fn is_ended(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    pub fn tokenize(&self) -> Vec<String> {
        todo!();
    }
}

#[cfg(test)]
mod interpreter_test {
    use super::Scanner;


    #[test]
    fn tokenize_file() {
        let file = r#"
            let some = "kwakky";
            let some_2 = "kwak ky ";
        "#;

        let mut scan = Scanner::<String>::new(file.into());
        assert_eq!(scan.)
    }
}