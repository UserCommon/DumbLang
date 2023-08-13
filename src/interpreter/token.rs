#![allow(dead_code)]
use std::fmt::{Display, Debug};
extern crate lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-char tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two char tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, Str(String), Number(f32),

    // Keywords
    And, Class, If, Or, Else, False, True, Fn,
    For, While, Nil, Print, Return, Super, This,
    Let,

    Eof
}

impl Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

use std::collections::HashMap;
use TokenType::*;

lazy_static::lazy_static! {
	pub static ref KEYWORDS: HashMap<String, TokenType> = { 
		let mut m = HashMap::new();
		m.insert("and".to_string(), And);
		m.insert("or".to_string(), Or);
		m.insert("class".to_string(), Class);
		m.insert("fn".to_string(), TokenType::Fn);
		m.insert("if".to_string(), If);
		m.insert("else".to_string(), Else);
		m.insert("true".to_string(), True);
		m.insert("false".to_string(), False);
		m.insert("for".to_string(), For);
		m.insert("while".to_string(), While);
		m.insert("nil".to_string(), Nil);
		m.insert("print".to_string(), Print);
		m.insert("return".to_string(), Return);
		m.insert("super".to_string(), Super);
		m.insert("this".to_string(), TokenType::This);
		m.insert("let".to_string(), Let);

		m
	};
}
// literal: FIXME dont need it because enums can store values
// and i assume, that u can just use enums...
pub struct Token {
	token_type: TokenType,
	lexeme: String,
	line: u32,
}

impl Token {
	pub fn new<S>(token_t: TokenType, lexeme: S, line: u32) -> Self
	where 
		S: Into<String>	
	{

		Self {
			token_type : token_t,
			lexeme : lexeme.into(),
			line : line
		}
	}
}

impl Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}