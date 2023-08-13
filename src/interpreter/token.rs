#![allow(dead_code)]
use std::fmt::{Display, Debug};

#[derive(Debug)]
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
    Identifier, Str(String), Number,

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


// literal: FIXME dont need it because enums can store values
// and i assume, that u can just use enums...
pub struct Token {
	token_type: TokenType,
	lexeme: String,
	line: u32,
}

impl Token {
	pub fn new<S>(token_t: TokenType, lexeme: S, literal: Option<T>, line: u32) -> Self
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
		match self.literal {
			Some(lit) => write!(f, "{} {} {}", self.token_type, self.lexeme, self.line),
			None => write!(f, "{} {} {} {}", self.token_type, self.lexeme, " ", self.line),
		}
    }
}