mod scanner;

use std::fs::read_to_string;
use std::io;

use scanner::Scanner;

fn tokenize_line<'a>(line: &str) {
    let mut scanner = Scanner::new(line);

    println!(">>>");
    let tokens = scanner.scan_tokens().unwrap();
    println!("count: {}", tokens.len());
    for t in tokens {
        print!("{:?} ", t);
    }

}

fn main() -> io::Result<()> {
    let input = read_to_string("./src/main.rs")?;

    println!(">>>");
    for (n, line) in input.lines().enumerate() {
        println!("{n:03}: ");
        tokenize_line(line);
    }
    Ok(())
}
