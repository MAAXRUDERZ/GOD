use colored::*;

use crate::formatter::format_placeholders;
use crate::model::CommandDoc;
use crate::ui::{
    colors::{BOLD, HACKER_GREEN, RESET},
    render,
};

pub fn render(doc: &CommandDoc, show_all_flags: bool) {
    println!();

    render::header(&doc.name);

    println!("{}", doc.description.white());
    println!();

    if !doc.examples.is_empty() {
        render::section("Examples");
        println!();

        for (i, example) in doc.examples.iter().enumerate() {
            println!("{}", example.description.bold());

            let command = format_placeholders(&example.command);

            println!("  > {HACKER_GREEN}{BOLD}{command}{RESET}");

            if i + 1 != doc.examples.len() {
                println!();
            }
        }

        println!();
    }

    if !doc.flags.is_empty() {
        render::section("Important Flags");
        println!();

        let total_flags = doc.flags.len();

        let shown_flags = if show_all_flags {
            total_flags
        } else {
            total_flags.min(8)
        };

        for flag in doc.flags.iter().take(shown_flags) {
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

            println!("{}", names.bright_cyan().bold());
            println!("    {}", flag.description.white());
            println!();
        }

        if !show_all_flags && total_flags > shown_flags {
            println!(
                "{}",
                format!("Showing {} of {} flags", shown_flags, total_flags)
                    .bright_black()
            );

            println!(
                "{}",
                format!("Run: god {} --all-flags", doc.name.to_lowercase())
                    .bright_black()
            );

            println!();
        }
    }

    if !doc.warnings.is_empty() {
        render::section("Warnings");
        println!();

        for warning in &doc.warnings {
            println!("{}", warning.red().bold());
        }

        println!();
    }

    if !doc.related.is_empty() {
        render::section("Related Commands");
        println!();

        for related in &doc.related {
            println!("• {}", related.green());
        }

        println!();
    }
}