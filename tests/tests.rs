use std::{fs, io};

use prism_rs::Prism;

#[test]
fn test() -> io::Result<()> {
    let source = fs::read_to_string("./src/lib.rs")?;

    println!(">>> Without line numbers");
    Prism::new(&source).render_to_console();

    println!(">>> With line numbers");
    Prism::new(&source).render_to_console_with_line_num();
    Ok(())
}

#[test]
fn test_block_string() {
    let source = r#"
    // line comment
    fn main() {     // main
        let a = 10; // a = 10
        let b = 20; // b = 20
        println!("a + b = {}", a + b);
    }
    "#;

    println!(">>> Without line numbers");
    Prism::new(source).render_to_console();

    println!(">>> With line numbers");
    Prism::new(source).render_to_console_with_line_num();
}
