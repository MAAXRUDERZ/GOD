#[derive(Debug)]
pub struct Example {
    pub description: String,
    pub command: String,
}

#[derive(Debug)]
pub struct CommandDoc {
    pub name: String,
    pub description: String,
    pub examples: Vec<Example>,
}