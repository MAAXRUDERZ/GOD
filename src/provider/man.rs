use std::fs::File;
use std::io::Read;
use std::process::Command;

use flate2::read::GzDecoder;

/// Returns the path to a command's man page.
///
/// Example:
/// ls -> /usr/share/man/man1/ls.1.gz
pub fn get_man_path(command: &str) -> Option<String> {
    let output = Command::new("man")
        .arg("-w")
        .arg(command)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(
        String::from_utf8(output.stdout)
            .ok()?
            .trim()
            .to_string(),
    )
}

/// Loads and decompresses the raw man page source.
pub fn load_man_source(command: &str) -> Option<String> {
    let path = get_man_path(command)?;

    let file = File::open(path).ok()?;
    let mut decoder = GzDecoder::new(file);

    let mut content = String::new();
    decoder.read_to_string(&mut content).ok()?;

    Some(content)
}