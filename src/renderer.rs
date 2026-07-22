use colored::*;

use crate::formatter::format_placeholders;
use crate::model::CommandDoc;

pub fn render(doc: &CommandDoc) {
    println!();

    println!("{}", "════════════════════════════════════".bright_blue());
    println!(
        "{}",
        format!("📖 {}", doc.name.to_uppercase())
            .bright_green()
            .bold()
    );
    println!("{}", "════════════════════════════════════".bright_blue());

    println!();
    println!("{}", doc.description.white());

    // ---------------- Examples ----------------

    if !doc.examples.is_empty() {
        println!();
        println!("{}", "Examples".yellow().bold());

        for example in &doc.examples {
            println!();
            println!("• {}", example.description.bold());

            let command = format_placeholders(&example.command);

            println!("    {}", command.cyan());
        }
    }

    // ---------------- Flags ----------------

    if !doc.flags.is_empty() {
        println!();
        println!("{}", "Flags".yellow().bold());
        println!();

        for flag in &doc.flags {
            let mut names = String::new();

            if let Some(short) = &flag.short {
                names.push_str(short);
            }

            if let Some(long) = &flag.long {
                if !names.is_empty() {
                    names.push_str(", ");
                }

                names.push_str(long);
            }

            println!("{:<35} {}", names.cyan(), flag.description.white());
        }
    }

    // ---------------- Warnings ----------------

    if !doc.warnings.is_empty() {
        println!();
        println!("{}", "Warnings".red().bold());

        for warning in &doc.warnings {
            println!("⚠ {}", warning);
        }
    }

    // ---------------- Related ----------------

    if !doc.related.is_empty() {
        println!();
        println!("{}", "Related Commands".yellow().bold());

        for related in &doc.related {
            println!("• {}", related.green());
        }
    }

    println!();
}