use std::{fs, io};

use prism_rs::Prism;

#[test]
fn test() -> io::Result<()> {
    let source = fs::read_to_string("./src/lib.rs")?;
    let mut prism = Prism::new(&source);
    prism.render_to_console();
    Ok(())
}

#[test]
fn test_block_string() {
    let source = r#"
    fn main() {
        let a = 10;
        let b = 20;
        println!("a + b = {}", a + b);
    }
    "#;
    let mut prism = Prism::new(source);
    prism.render_to_console();
}
