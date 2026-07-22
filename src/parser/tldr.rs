use crate::model::{CommandDoc, Example};

pub fn parse_tldr(content: &str) -> CommandDoc {
    let mut name = String::new();
    let mut description = String::new();
    let mut examples = Vec::new();

    let mut current_description: Option<String> = None;

    for line in content.lines() {
        // Command name
        if let Some(command) = line.strip_prefix("# ") {
            name = command.to_string();
            continue;
        }

        // Short description
        if description.is_empty() && line.starts_with("> ") {
            let text = line.trim_start_matches("> ");

            if !text.starts_with("More information") {
                description = text.to_string();
            }

            continue;
        }

        // Example description
        if let Some(text) = line.strip_prefix("- ") {
            current_description = Some(text.to_string());
            continue;
        }

        // Example command
        if line.starts_with('`') && line.ends_with('`') {
            if let Some(desc) = current_description.take() {
                let command = line.trim_matches('`').to_string();

                examples.push(Example {
                    description: desc,
                    command,
                });
            }
        }
    }

    CommandDoc {
    name,
    description,
    examples,

    flags: Vec::new(),
    warnings: Vec::new(),
    related: Vec::new(),
    }
}