use crate::ui::{
    colors::{BOLD, GOLD, RESET},
    layout::WIDTH,
};

pub fn header(title: &str) {
    let text = format!(" {} ", title.to_uppercase());

    let padding = WIDTH.saturating_sub(text.chars().count());
    let left = padding / 2;
    let right = padding - left;

    println!(
        "{GOLD}{BOLD}{}{text}{}{RESET}",
        "=".repeat(left),
        "=".repeat(right),
    );

    println!();
}

pub fn section(title: &str) {
    let text = format!(" {} ", title);

    let padding = WIDTH.saturating_sub(text.chars().count());
    let left = padding / 2;
    let right = padding - left;

    println!(
        "{GOLD}{BOLD}{}{text}{}{RESET}",
        "─".repeat(left),
        "─".repeat(right),
    );
}