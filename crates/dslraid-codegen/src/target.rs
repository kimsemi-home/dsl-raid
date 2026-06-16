#[derive(Debug, Clone, Copy)]
pub enum CodegenTarget {
    Rust,
    Go,
    TypeScript,
    Mermaid,
    Dot,
}

impl CodegenTarget {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Go => "go",
            Self::TypeScript => "ts",
            Self::Mermaid => "mmd",
            Self::Dot => "dot",
        }
    }
}
