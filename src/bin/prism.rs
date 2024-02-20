use std::{env, fs, io};

use prism_rs::Prism;

fn main() -> io::Result<()> {
    let files: Vec<String> = env::args().skip(1).collect();

    for f in files {
        println!(">>> {f}");
        let source = fs::read_to_string(f)?;
        println!("{}", Prism::new(&source).render_to_console_with_line_num());
    }
    Ok(())
}
