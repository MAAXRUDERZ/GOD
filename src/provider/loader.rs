use crate::model::{CommandDoc, Flag};
use crate::{parser, provider};

fn is_important_flag(flag: &Flag) -> bool {
    let mut names = String::new();

    if let Some(short) = &flag.short {
        names.push_str(short);
    }

    if let Some(long) = &flag.long {
        if !names.is_empty() {
            names.push(' ');
        }
        names.push_str(long);
    }

    let ignored = [
        "--help",
        "--version",
        "--author",
        "--color",
        "--hyperlink",
        "--quoting-style",
        "--time-style",
        "--indicator-style",
        "--show-control-chars",
        "--hide-control-chars",
        "--dired",
        "--full-time",
        "--block-size",
        "--tabsize",
        "--context",
        "--zero",
    ];

    !ignored.iter().any(|item| names.contains(item))
}

pub fn load(command: &str) -> Option<CommandDoc> {
    let mut doc = CommandDoc::default();
    let mut found = false;

    if let Ok(tldr_doc) = provider::tldr::load_documentation(command) {
        doc = tldr_doc;
        found = true;
    }

    if let Some(content) = provider::man::load_man_source(command) {
        let mut man_doc = parser::man::parse_man(&content);

        // Keep all meaningful flags, renderer decides how many to display
        man_doc.flags = man_doc
            .flags
            .into_iter()
            .filter(is_important_flag)
            .collect();

        if doc.description.is_empty() {
            doc.description = man_doc.description;
        }

        if doc.name.is_empty() {
            doc.name = man_doc.name;
        }

        doc.flags = man_doc.flags;
        doc.warnings = man_doc.warnings;
        doc.related = man_doc.related;

        found = true;
    }

    if found {
        Some(doc)
    } else {
        None
    }
}