use std::{fs, io};

use prism_rs::Prism;

#[test]
fn test() -> io::Result<()> {
    let source = fs::read_to_string("./src/lib.rs")?;
    let mut prism = Prism::new(&source);
    prism.render_to_console();
    Ok(())
}
