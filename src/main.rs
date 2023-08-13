#![warn(dead_code)]

mod utils;
mod interpreter;

use utils::*;
use interpreter::*;

use std::env;
use std::io::Result;


fn main() -> Result<()> {
    let mut interpreter = Interpreter::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        throw_help();
    } else if args.len() == 2 {
        interpreter.ExecuteFile(&args[1])?;
    } else {
        interpreter.Shell()?;
    }
    Ok(())
}
