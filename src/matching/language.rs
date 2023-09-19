#[derive(Debug)]
pub enum Language {
    Rust,
    NodeJS,
    Python,
    Custom(String),
}

impl Language {
    fn name(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::NodeJS => "NodeJS",
            Language::Python => "Python",
            Language::Custom(name) => name,
        }
    }
}
