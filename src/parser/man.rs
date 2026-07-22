use crate::model::{CommandDoc, Flag};

fn clean_option(text: &str) -> String {
    let mut result = text.to_string();

    // Remove font formatting
    result = result
        .replace("\\fB", "")
        .replace("\\fP", "")
        .replace("\\fR", "")
        .replace("\\fI", "");

    // Remove common roff escapes
    result = result
        .replace("\\-", "-")
        .replace("\\&", "")
        .replace("\\,", "")
        .replace("\\/", "")
        .replace("\\ ", " ");

    // Remove GNU groff escape sequences like \X'...'
    while let Some(start) = result.find("\\X'") {
        if let Some(end) = result[start + 3..].find('\'') {
            result.replace_range(start..=start + 3 + end, "");
        } else {
            break;
        }
    }

    // Collapse multiple spaces into one
    result = result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    result.trim().to_string()
}

pub fn parse_man(content: &str) -> CommandDoc {
    let mut flags = Vec::new();

    let mut lines = content.lines();

    let mut name = String::new();
    let mut description = String::new();

    let mut in_description = false;

    while let Some(line) = lines.next() {
        // ----------------------------
        // Parse NAME section
        // ----------------------------
        if line == ".SH NAME" {
            if let Some(next) = lines.next() {
                let next = clean_option(next);

                if let Some((cmd, summary)) = next.split_once(" \\- ") {
                    name = cmd.trim().to_string();

                    let mut summary = summary.trim().to_string();

                    // Capitalize first letter
                    if let Some(first) = summary.chars().next() {
                        summary.replace_range(
                            0..first.len_utf8(),
                            &first.to_uppercase().to_string(),
                        );
                    }

                    // Add period if missing
                    if !summary.ends_with('.') {
                        summary.push('.');
                    }

                    description = summary;
                } else {
                    name = next.trim().to_string();
                }
            }

            continue;
        }

        // ----------------------------
        // Enter DESCRIPTION section
        // ----------------------------
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

        // ----------------------------
        // Parse flags
        // ----------------------------
        if line == ".TP" {
            let option_line = match lines.next() {
                Some(l) => clean_option(l),
                None => break,
            };

            let desc = match lines.next() {
                Some(l) => clean_option(l),
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