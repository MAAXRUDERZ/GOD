mod formatter;
mod model;
mod parser;
mod provider;
mod renderer;
mod ui;

use clap::Parser;

/// GOD - Better than man pages
#[derive(Parser, Debug)]
#[command(name = "god")]
#[command(version = "0.1.0")]
#[command(about = "The modern Linux command guide")]
struct Cli {
    /// Command to look up
    command: Option<String>,

    /// Show every available flag
    #[arg(long)]
    all_flags: bool,
}

fn main() {
    let cli = Cli::parse();

    let Some(cmd) = cli.command else {
        ui::logo::print_logo();

        println!();
        println!("Usage:");
        println!("  god <command>");
        println!("  god <command> --all-flags");
        println!();

        return;
    };

    ui::logo::print_logo();
    println!("Command: {}\n", cmd);

    match provider::load(&cmd) {
        Some(doc) => {
            renderer::render(&doc, cli.all_flags);
        }

        None => {
            println!("No documentation found for '{}'.", cmd);
        }
    }
}