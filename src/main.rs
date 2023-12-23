mod scanner;

use std::fs::read_to_string;
use std::io;

use scanner::Scanner;

fn main() -> io::Result<()> {
    let input = read_to_string("./src/scanner.rs")?;
    let mut scanner = Scanner::new(input.as_str());
    let result = scanner.scan();
    for l in result {
        println!("{l:?}");
    }
    Ok(())
}
