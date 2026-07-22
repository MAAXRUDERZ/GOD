pub fn format_placeholders(text: &str) -> String {
    text.replace("{{", "<")
        .replace("}}", ">")
}