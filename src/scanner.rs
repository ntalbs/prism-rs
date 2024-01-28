use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Whitespace(String),
    NewLine(),
    Punctuation(String),
    Number(String),
    String(String),
    LineComment(String),
    BlockComment(String),
    Name(String),
    Keyword(String),
    Eof,
}

pub struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
            current_char: Option::None,
        }
    }

    pub(crate) fn scan(&mut self) -> Vec<Token> {
        fn break_by_line(str: String) -> Vec<Token> {
            let mut bcl = str.split('\n').map(|line| Token::BlockComment(line.into()));
            let mut ret = Vec::new();
            if let Some(line) = bcl.next() {
                ret.push(line);
            }
            for line in bcl {
                ret.push(Token::NewLine());
                ret.push(line);
            }
            ret
        }

        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let token = self.next_token();
            match token {
                Token::Eof => break,
                Token::BlockComment(s) => tokens.append(&mut break_by_line(s)),
                _ => tokens.push(token),
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.current_char = self.advance();
        match self.current_char {
            Some(' ') => self.whitespace(),
            Some('\n') => self.newline(),
            Some('"') => self.string(),
            Some('/') => match self.peek() {
                Some('/') => self.line_comment(),
                Some('*') => self.block_comment(),
                Some(_) => self.punctuation(),
                None => Token::Eof,
            },
            Some(c) if Self::is_punctuation(c) => self.punctuation(),
            Some(c) if c.is_ascii_digit() => self.number(),
            Some(_) => self.name_or_keyword(),
            None => Token::Eof,
        }
    }

    fn whitespace(&mut self) -> Token {
        let mut buf = String::from(self.current_char.unwrap());
        while let Some(' ') = self.peek() {
            buf.push(' ');
            self.advance();
        }
        Token::Whitespace(buf)
    }

    fn newline(&mut self) -> Token {
        Token::NewLine()
    }

    fn punctuation(&mut self) -> Token {
        let mut buf = String::from(self.current_char.unwrap());

        while let Some(&c) = self.peek() {
            if !Self::is_punctuation(c) {
                break;
            }
            buf.push(c);
            self.advance();
        }
        Token::Punctuation(buf)
    }

    fn number(&mut self) -> Token {
        let mut buf = String::from(self.current_char.unwrap());
        while let Some(&c) = self.peek() {
            if !c.is_ascii_digit() && c != '.' && c != '_' {
                break;
            }
            buf.push(c);
            self.advance();
        }
        Token::Number(buf)
    }

    fn string(&mut self) -> Token {
        let mut buf = String::from("\"");

        let mut prev_char: char = '\n';
        while let Some(&c) = self.peek() {
            match c {
                '\n' => break,
                '"' if prev_char != '\\' => break, // if prev_char=='\\', then escaped
                _ => {
                    buf.push(c);
                    prev_char = c;
                    self.advance();
                }
            }
        }
        match self.advance() {
            Some('\n') => (), // non-terminated string
            Some('"') => buf.push('"'),
            Some(_) => (), // shouldn't come here
            None => (),    // EOF, do nothing
        }
        Token::String(buf)
    }

    fn line_comment(&mut self) -> Token {
        let mut buf = String::from(self.current_char.unwrap());

        while let Some(&c) = self.peek() {
            if c == '\n' {
                break;
            }
            buf.push(c);
            self.advance();
        }
        Token::LineComment(buf)
    }

    fn block_comment(&mut self) -> Token {
        let mut buf = String::from(self.current_char.unwrap());

        while let Some(&c) = self.peek() {
            buf.push(c);
            self.current_char = self.advance();
            if self.current_char == Some('*') && self.peek() == Some(&'/') {
                self.current_char = self.advance();
                buf.push('/');
                break;
            }
        }

        Token::BlockComment(buf)
    }

    fn name_or_keyword(&mut self) -> Token {
        let mut buf = String::from(self.current_char.unwrap());

        while let Some(&c) = self.peek() {
            if c == ' ' || c == '\n' || !Self::is_valid_for_identifier(c) {
                break;
            }
            buf.push(c);
            self.advance();
        }
        if Self::is_keyword(&buf) {
            Token::Keyword(buf)
        } else {
            Token::Name(buf)
        }
    }

    fn is_valid_for_identifier(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_punctuation(c: char) -> bool {
        match c {
            c if c.is_alphanumeric() => false,
            c if c.is_ascii_whitespace() => false,
            '"' => false,
            _ => true,
        }
    }

    fn is_keyword(name: &str) -> bool {
        let keywords: HashSet<&str> = HashSet::<_>::from_iter([
            "Self",
            "abstract",
            "as",
            "async",
            "await",
            "become",
            "box",
            "break",
            "const",
            "continue",
            "crate",
            "do",
            "dyn",
            "dyn",
            "else",
            "enum",
            "extern",
            "false",
            "final",
            "fn",
            "for",
            "if",
            "impl",
            "in",
            "let",
            "loop",
            "macro",
            "macro_rules",
            "match",
            "mod",
            "move",
            "mut",
            "override",
            "priv",
            "pub",
            "ref",
            "return",
            "self",
            "static",
            "struct",
            "super",
            "trait",
            "true",
            "try",
            "type",
            "typeof",
            "union",
            "unsafe",
            "unsized",
            "use",
            "virtual",
            "where",
            "while",
            "yield",
        ]);
        keywords.contains(name)
    }

    fn advance(&mut self) -> Option<char> {
        self.iter.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::{Scanner, Token};

    /// macro for single line input
    macro_rules! testln {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut scanner = Scanner::new($input);
                let tokens = scanner.scan();
                assert_eq!(tokens, $expected);
            }
        };
    }

    macro_rules! ws {
        ($input:literal) => {
            Token::Whitespace($input.into())
        };
    }

    macro_rules! nl {
        () => {
            Token::NewLine()
        };
    }

    macro_rules! pt {
        ($input:literal) => {
            Token::Punctuation($input.into())
        };
    }

    macro_rules! nu {
        ($input:literal) => {
            Token::Number($input.into())
        };
    }

    macro_rules! st {
        ($input:literal) => {
            Token::String($input.into())
        };
    }

    macro_rules! lc {
        ($input:literal) => {
            Token::LineComment($input.into())
        };
    }

    macro_rules! bc {
        ($input:literal) => {
            Token::BlockComment($input.into())
        };
    }

    macro_rules! nm {
        ($input:literal) => {
            Token::Name($input.into())
        };
    }

    macro_rules! kw {
        ($input:literal) => {
            Token::Keyword($input.into())
        };
    }

    testln!(single_token_ws, "    ", vec![ws!("    ")]);
    testln!(single_token_pt, "+=", vec![pt!("+=")]);
    testln!(single_token_nu_1, "12345", vec![nu!("12345")]);
    testln!(single_token_nu_2, "12.345", vec![nu!("12.345")]);
    testln!(single_token_nu_3, "12_345", vec![nu!("12_345")]);
    testln!(single_token_st, "\"hello\"", vec!(st!("\"hello\"")));
    testln!(single_token_lc, "// comment", vec!(lc!("// comment")));
    testln!(single_token_kw_1, "fn", vec![kw!("fn")]);
    testln!(single_token_kw_3, "self", vec![kw!("self")]);
    testln!(single_token_kw_2, "Self", vec![kw!("Self")]);
    testln!(single_token_nm, "name", vec![nm!("name")]);

    testln!(
        single_line_let,
        "let a = 10;",
        vec![
            kw!("let"),
            ws!(" "),
            nm!("a"),
            ws!(" "),
            pt!("="),
            ws!(" "),
            nu!("10"),
            pt!(";")
        ]
    );

    testln!(
        str_argument,
        "println!(\"hello, world\");",
        vec![
            nm!("println"),
            pt!("!("),
            st!("\"hello, world\""),
            pt!(");"),
        ]
    );

    testln!(
        str_escaped_double_quotation,
        r#""{\"a\": 10}""#,
        vec![st!("\"{\\\"a\\\": 10}\"")]
    );

    testln!(
        block_comment,
        r#"/*
        * Block comment line 1.
        * Block comment line 2.
        */
        let a = 10;"#,
        vec![
            bc!("/*"),
            nl!(),
            bc!("        * Block comment line 1."),
            nl!(),
            bc!("        * Block comment line 2."),
            nl!(),
            bc!("        */"),
            nl!(),
            ws!("        "),
            kw!("let"),
            ws!(" "),
            nm!("a"),
            ws!(" "),
            pt!("="),
            ws!(" "),
            nu!("10"),
            pt!(";")
        ]
    );
}
