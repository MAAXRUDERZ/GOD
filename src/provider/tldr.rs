use std::{
    fs,
    io,
    path::PathBuf,
};

use crate::model::CommandDoc;
use crate::parser::parse_tldr;

fn god_data_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".local")
        .join("share")
        .join("god")
}

/// Searches for a TLDR page in the supported directories.
fn find_page(command: &str) -> Option<PathBuf> {
    let base = god_data_dir().join("tldr");

    let locations = [
        base.join("pages").join("common").join(format!("{}.md", command)),
        base.join("pages").join("linux").join(format!("{}.md", command)),
    ];

    for path in locations {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Loads documentation for a command.
pub fn load_documentation(command: &str) -> io::Result<CommandDoc> {
    let path = find_page(command)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Command not found"))?;

    let content = fs::read_to_string(path)?;

    Ok(parse_tldr(&content))
}