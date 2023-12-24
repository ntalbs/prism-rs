pub(crate) trait Color {
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;

    fn bright_black(&self) -> String;
    fn bright_red(&self) -> String;
    fn bright_green(&self) -> String;
    fn bright_yellow(&self) -> String;
    fn bright_blue(&self) -> String;
    fn bright_magenta(&self) -> String;
    fn bright_cyan(&self) -> String;
    fn bright_white(&self) -> String;
}

impl Color for str {
    fn black(&self) -> String {
        format!("\x1b[30m{self}\x1b[0m")
    }

    fn red(&self) -> String {
        format!("\x1b[31m{self}\x1b[0m")
    }

    fn green(&self) -> String {
        format!("\x1b[32m{self}\x1b[0m")
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{self}\x1b[0m")
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{self}\x1b[0m")
    }

    fn magenta(&self) -> String {
        format!("\x1b[35m{self}\x1b[0m")
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{self}\x1b[0m")
    }

    fn white(&self) -> String {
        format!("\x1b[37m{self}\x1b[0m")
    }

    fn bright_black(&self) -> String {
        format!("\x1b[30;1m{self}\x1b[0m")
    }

    fn bright_red(&self) -> String {
        format!("\x1b[31;1m{self}\x1b[0m")
    }

    fn bright_green(&self) -> String {
        format!("\x1b[32;1m{self}\x1b[0m")
    }

    fn bright_yellow(&self) -> String {
        format!("\x1b[33;1m{self}\x1b[0m")
    }

    fn bright_blue(&self) -> String {
        format!("\x1b[34;1m{self}\x1b[0m")
    }

    fn bright_magenta(&self) -> String {
        format!("\x1b[35;1m{self}\x1b[0m")
    }

    fn bright_cyan(&self) -> String {
        format!("\x1b[36;1m{self}\x1b[0m")
    }

    fn bright_white(&self) -> String {
        format!("\x1b[37;1m{self}\x1b[0m")
    }
}
