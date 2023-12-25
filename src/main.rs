mod renderer;
mod scanner;

use std::fs::read_to_string;
use std::io;

use renderer::render_to_console;
use scanner::Scanner;

fn main() -> io::Result<()> {
    let input = read_to_string("./src/scanner.rs")?;
    let mut scanner = Scanner::new(input.as_str());
    let result = scanner.scan();

    for l in &result {
        println!("{l:?}");
    }

    render_to_console(result);

    Ok(())
}
