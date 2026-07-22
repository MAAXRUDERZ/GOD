use colored::*;

use crate::formatter::format_placeholders;
use crate::model::CommandDoc;
use crate::ui::{
    colors::{BOLD, HACKER_GREEN, RESET},
    render,
};

pub fn render(doc: &CommandDoc) {
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

            println!("{}", names.bright_cyan().bold());
            println!("    {}", flag.description.white());
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