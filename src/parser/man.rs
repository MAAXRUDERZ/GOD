use crate::model::{CommandDoc, Flag};

fn clean_option(line: &str) -> String {
    let mut result = line
        .replace("\\fB", "")
        .replace("\\fP", "")
        .replace("\\-", "-");

    while let Some(start) = result.find("\\X'") {
        if let Some(end) = result[start + 3..].find('\'') {
            result.replace_range(start..=start + 3 + end, "");
        } else {
            break;
        }
    }

    result
}

pub fn parse_man(content: &str) -> CommandDoc {
    let mut flags = Vec::new();

    let mut lines = content.lines();
    let mut in_description = false;

    let mut name = String::new();
    let mut description = String::new();

    while let Some(line) = lines.next() {
        // Parse command name
        if line == ".SH NAME" {
            if let Some(next) = lines.next() {
                if let Some((cmd, _)) = next.split_once(" \\- ") {
                    name = cmd.trim().to_string();
                } else {
                    name = next.trim().to_string();
                }
            }
            continue;
        }

        // Enter DESCRIPTION section
        if line == ".SH DESCRIPTION" {
            in_description = true;
            continue;
        }

        if !in_description {
            continue;
        }

        // Stop before Exit status
        if line == ".SS \"Exit status:\"" {
            break;
        }

        // First non-empty line becomes the description
        if description.is_empty()
            && !line.starts_with('.')
            && !line.trim().is_empty()
        {
            description = clean_option(line).trim().to_string();
            continue;
        }

        // Parse flags
        if line == ".TP" {
            let option_line = match lines.next() {
                Some(l) => clean_option(l),
                None => break,
            };

            let desc = match lines.next() {
                Some(l) => clean_option(l).trim().to_string(),
                None => String::new(),
            };

            let mut short = None;
            let mut long = None;

            for part in option_line.split(',') {
                let part = part.trim();

                if part.starts_with("--") {
                    long = Some(part.to_string());
                } else if part.starts_with('-') {
                    short = Some(part.to_string());
                }
            }

            flags.push(Flag {
                short,
                long,
                description: desc,
            });
        }
    }

    CommandDoc {
        name,
        description,
        examples: Vec::new(),
        flags,
        warnings: Vec::new(),
        related: Vec::new(),
    }
}