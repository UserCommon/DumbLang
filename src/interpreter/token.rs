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
pub struct Token<T: Display> {
	token_type: TokenType,
	lexeme: String,
	literal: Option<T>,
	line: u32,
}

impl<T: Display> Token<T> {
	pub fn new<S>(token_t: TokenType, lexeme: S, literal: Option<T>, line: u32) -> Self
	where 
		S: Into<String>	
	{

		Self {
			token_type : token_t,
			lexeme : lexeme.into(),
			literal : literal,
			line : line
		}
	}
}

impl<T: Display> Debug for Token<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.literal {
			Some(lit) => write!(f, "{} {} {} {}", self.token_type, self.lexeme, lit, self.line),
			None => write!(f, "{} {} {} {}", self.token_type, self.lexeme, " ", self.line),
		}
    }
}