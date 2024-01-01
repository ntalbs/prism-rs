mod renderer;
mod scanner;

use renderer::render_to_console;
use scanner::Scanner;

pub struct Prism<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Prism<'a> {
    pub fn new(input: &'a str) -> Self {
        Prism {
            scanner: Scanner::new(input),
        }
    }

    pub fn render_to_console(&mut self) {
        let scanned = self.scanner.scan();
        render_to_console(scanned);
    }
}
