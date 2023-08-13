#![allow(dead_code)]

pub fn throw_help() {
    eprintln!("Usage: dl [file]");
    std::process::exit(64);
}

pub fn throw_error(line: u32, err: Result<(), String>) {
    throw_detailed_error(line, err, "somewhere".into());
}

fn throw_detailed_error(line: u32, err: Result<(), String>, place: String) {
    eprintln!("Error: {error} in {place}\n
              \t line: {line} | \n",
              error = err.err().unwrap(), place = place, line = line);
}

// /// Separating ; and \n for getting split(" ") work
// pub fn prepare_src<T: Into<String>>(str: T) -> String {
//     let mut result = String::from("");
//     for char in str.into().chars() {
//         if char == ';' || char == '\n'
//         || char == '\t'
//         {
//             result.push(' ');
//         }
//         result.push(char);
//     }
//     result
// }

// #[cfg(test)]
// mod test {
//     #[test]
//     fn preparing() {
//         assert_eq!(super::prepare_src("var something = l;\nvar two = xd;"),
//                    "var something");
//     }
// }