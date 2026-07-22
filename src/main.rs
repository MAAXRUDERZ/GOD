mod model;
mod provider;
mod renderer;
mod parser;
mod formatter;

use clap::Parser;

/// GOD - Better than man pages
#[derive(Parser, Debug)]
#[command(name = "god")]
#[command(version = "0.1.0")]
#[command(about = "The modern Linux command guide")]
struct Cli {
    /// Command to look up
    command: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let Some(cmd) = cli.command else {
        println!("Welcome to GOD!");
        println!("Usage:");
        println!("    god <command>");
        return;
    };

    println!("GOD v0.1.0");
    println!("Command: {}\n", cmd);

    match provider::load(&cmd) {
        Some(doc) => {
            renderer::render(&doc);
        }

        None => {
            println!("No documentation found for '{}'.", cmd);
        }
    }
}