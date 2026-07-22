#[derive(Debug, Default)]
pub struct Example {
    pub description: String,
    pub command: String,
}

#[derive(Debug, Default)]
pub struct Flag {
    pub short: Option<String>,
    pub long: Option<String>,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct CommandDoc {
    pub name: String,
    pub description: String,

    pub examples: Vec<Example>,

    pub flags: Vec<Flag>,
    pub warnings: Vec<String>,
    pub related: Vec<String>,
}