use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

use crate::model::{CommandDoc, Example};

/// Searches for a TLDR page in the supported directories.
fn find_page(command: &str) -> Option<PathBuf> {
    let locations = [
        format!("tldr/pages/common/{}.md", command),
        format!("tldr/pages/linux/{}.md", command),
    ];

    for location in locations {
        let path = Path::new(&location);

        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    None
}

/// Loads documentation for a command.
///
/// Parsing will be implemented next.
pub fn load_documentation(command: &str) -> io::Result<CommandDoc> {
    let path = find_page(command)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Command not found"))?;

    let content = fs::read_to_string(path)?;

    println!("Loaded page successfully!");
    println!("-------------------------");
    println!("{}", content);

    Ok(CommandDoc {
        name: String::new(),
        description: String::new(),
        examples: Vec::<Example>::new(),
    })
}
