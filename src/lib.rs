mod renderer;
mod scanner;

use renderer::{render_to_console, render_to_console_with_line_num, render_to_html};
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

    pub fn render_to_console(&mut self) -> String {
        let scanned = self.scanner.scan();
        render_to_console(&scanned)
    }

    pub fn render_to_console_with_line_num(&mut self) -> String {
        let scanned = self.scanner.scan();
        render_to_console_with_line_num(&scanned)
    }

    pub fn render_to_html(&mut self) -> String {
        let scanned = self.scanner.scan();
        render_to_html(&scanned)
    }
}
