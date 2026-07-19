mod model;
mod provider;
mod renderer;

use clap::Parser;
use provider::load_documentation;
use renderer::render;

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

    match cli.command {
        Some(cmd) => {
            println!("GOD v0.1.0");
            println!("Command: {}\n", cmd);

            match load_documentation(&cmd) {
                Ok(doc) => {
                    render(&doc);
                }

                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        None => {
            println!("Welcome to GOD!");
            println!("Usage:");
            println!("    god <command>");
        }
    }
}