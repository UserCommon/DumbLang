mod scanner;
mod token;

use std::path;
use std::fs::{self};
use std::io::{Result, Read, self};

use scanner::*;

pub enum States {
	Running,
	Error,
	Ended
}

pub struct Interpreter {
	state: States
}

impl Interpreter {
	pub fn new() -> Self {
		Self {
			state: States::Running
		}
	}

	pub fn Execute<T: Into<String>>(&mut self, source: T) -> Result<()> {
		let source: String = source.into();
		let mut scanner: Scanner = Scanner::new(source);
		let tokenized = scanner.tokenize();
		
		for token in tokenized {
			println!("{:?}", token);
		}

		Ok(())
	}

	/// Executes code from file
	pub fn ExecuteFile<T: AsRef<path::Path>>(&mut self, path: T) -> Result<()> {
		let mut file = fs::File::open(&path)?;
		// let metadata = fs::metadata(&path)?;
		// let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
		// file.read(&mut buffer)?;
		
		let mut string = String::from("");
		file.read_to_string(&mut string)?;
		
		self.Execute(string)?;

		use States::*;
		match self.state {
			Error => {std::process::exit(666);},
			Ended => {std::process::exit(0);}
			_ => Ok(())
		}
	}

	pub fn Shell(&mut self) -> Result<()> {
		let stdin = io::stdin();

		for line in stdin.lines() {
			print!("> ");
			let line = line.unwrap().to_string();
			// TODO: handle crtl + d or idunno
			self.Execute(line)?;
			self.state = States::Running;
		}

		Ok(())
	}
}
