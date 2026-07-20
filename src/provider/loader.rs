use crate::model::CommandDoc;
use crate::{parser, provider};

pub fn load(command: &str) -> Option<CommandDoc> {
    let mut doc = CommandDoc::default();
    let mut found = false;

    // ---------- TLDR ----------
    if let Ok(tldr_doc) = provider::tldr::load_documentation(command) {
        doc = tldr_doc;
        found = true;
    }

    // ---------- MAN ----------
    if let Some(content) = provider::man::load_man_source(command) {
        let man_doc = parser::man::parse_man(&content);

        // Fill missing description
        if doc.description.is_empty() {
            doc.description = man_doc.description;
        }

        if doc.name.is_empty() {
            doc.name = man_doc.name;
        }

        // Merge flags
        doc.flags = man_doc.flags;

        // Merge warnings
        doc.warnings = man_doc.warnings;

        // Merge related
        doc.related = man_doc.related;

        found = true;
    }

    if found {
        Some(doc)
    } else {
        None
    }
}
