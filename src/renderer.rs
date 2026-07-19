use colored::*;
use crate::model::CommandDoc;

pub fn render(doc: &CommandDoc) {
    println!();

    println!("{}", "════════════════════════════════════".bright_blue());
    println!("{}", format!("📖 {}", doc.name.to_uppercase()).bright_green().bold());
    println!("{}", "════════════════════════════════════".bright_blue());

    println!();
    println!("{}", doc.description.white());

    println!();
    println!("{}", "Examples".yellow().bold());

    for example in &doc.examples {
        println!();
        println!("• {}", example.description.bold());
        println!("    {}", example.command.cyan());
    }

    println!();
}