mod contract;

pub use contract::CodegenContract;

#[derive(Debug, Clone, Copy)]
pub enum CodegenTarget {
    Rust,
    Go,
    TypeScript,
    Mermaid,
    Dot,
}

impl CodegenTarget {
    pub const ALL: [Self; 5] = [
        Self::Rust,
        Self::Go,
        Self::TypeScript,
        Self::Mermaid,
        Self::Dot,
    ];

    pub fn contract(self) -> CodegenContract {
        contract::for_target(self)
    }

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
