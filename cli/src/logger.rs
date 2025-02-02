use std::process;

use ansi_term::{Colour, Style};

fn pretty_print(color: Colour, level: &str, content: &str) {
    println!(
        "{} {}",
        Style::new().fg(color).paint(format!("[{}]", level)),
        content
    );
}

pub fn info(content: &str) {
    pretty_print(Colour::Blue, "INFO", content);
}

pub fn success(content: &str) {
    pretty_print(Colour::Green, "SUCCESS", content);
}

pub fn warn(content: &str) {
    pretty_print(Colour::Yellow, "WARN", content)
}

pub fn error(content: &str) {
    pretty_print(Colour::Red, "ERROR", content);
    process::exit(1);
}
