pub fn format_placeholders(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' && chars.peek() == Some(&'{') {
            chars.next(); // consume second {

            let mut placeholder = String::new();

            while let Some(c) = chars.next() {
                if c == '}' && chars.peek() == Some(&'}') {
                    chars.next(); // consume second }
                    break;
                }

                placeholder.push(c);
            }

            result.push_str(&format_placeholder(&placeholder));
        } else {
            result.push(ch);
        }
    }

    clean_text(&result)
}

fn format_placeholder(content: &str) -> String {
    let content = content.trim();

    // {{[-a|--all]}}
    if content.starts_with('[') && content.ends_with(']') {
        let inner = &content[1..content.len() - 1];

        if let Some(first) = inner.split('|').next() {
            return first.trim().to_string();
        }
    }

    // {{file}}
    format!("<{}>", content)
}

fn clean_text(text: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = text.chars().collect();

    let mut i = 0;

    while i < chars.len() {
        // Convert [l] -> l
        if chars[i] == '['
            && i + 2 < chars.len()
            && chars[i + 2] == ']'
        {
            out.push(chars[i + 1]);
            i += 3;
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    // Remove trailing " */"
    out = out.trim_end().to_string();

    if out.ends_with(" */") {
        out.truncate(out.len() - 3);
    }

    if out.ends_with("*/") {
        out.truncate(out.len() - 2);
    }

    // Remove trailing colon
    if out.ends_with(':') {
        out.pop();
    }

    out
}