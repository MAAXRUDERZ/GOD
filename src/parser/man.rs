use crate::model::{CommandDoc, Flag};

fn clean_option(text: &str) -> String {
    let mut result = text.to_string();


    result = result
        .replace("\\fB", "")
        .replace("\\fP", "")
        .replace("\\fR", "")
        .replace("\\fI", "");


    result = result
        .replace("\\-", "-")
        .replace("\\&", "")
        .replace("\\,", "")
        .replace("\\/", "")
        .replace("\\ ", " ");

    while let Some(start) = result.find("\\X'") {
        if let Some(end) = result[start + 3..].find('\'') {
            result.replace_range(start..=start + 3 + end, "");
        } else {
            break;
        }
    }


    result = result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    result.trim().to_string()
}

pub fn parse_man(content: &str) -> CommandDoc {
    let mut flags = Vec::new();

    let lines: Vec<&str> = content.lines().collect();

    let mut name = String::new();
    let mut description = String::new();

    let mut in_description = false;

    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];


        if line == ".SH NAME" {
            i += 1;

            if i < lines.len() {
                let next = clean_option(lines[i]);

                if let Some((cmd, summary)) = next.split_once(" \\- ") {
                    name = cmd.trim().to_string();

                    let mut summary = summary.trim().to_string();

                    if let Some(first) = summary.chars().next() {
                        summary.replace_range(
                            0..first.len_utf8(),
                            &first.to_uppercase().to_string(),
                        );
                    }

                    if !summary.ends_with('.') {
                        summary.push('.');
                    }

                    description = summary;
                } else {
                    name = next.trim().to_string();
                }
            }

            i += 1;
            continue;
        }


        if line == ".SH DESCRIPTION" {
            in_description = true;
            i += 1;
            continue;
        }

        if !in_description {
            i += 1;
            continue;
        }

        // Stop before Exit status
        if line == ".SS \"Exit status:\"" {
            break;
        }


        if line == ".TP" {
            i += 1;
            if i >= lines.len() {
                break;
            }

            let option_line = clean_option(lines[i]);

            i += 1;
            if i >= lines.len() {
                break;
            }

            let desc = clean_option(lines[i]);

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

        i += 1;
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