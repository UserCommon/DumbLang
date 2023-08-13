#![allow(dead_code)]

pub fn throw_help() {
    eprintln!("Usage: dl [file]");
    std::process::exit(64);
}

pub fn throw_error(line: u32, err: Result<(), String>) {
    throw_detailed_error(line, err, "somewhere".into());
}

fn throw_detailed_error(line: u32, err: Result<(), String>, place: String) {
    eprintln!("Error: {error} in {place}\n\n
              \t {line} | \n",
              error = err.err().unwrap(), place = place, line = line);
}
