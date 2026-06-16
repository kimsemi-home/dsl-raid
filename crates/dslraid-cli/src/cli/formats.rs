use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum DiffFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum RenderFormat {
    Svg,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliCodegenTarget {
    Rust,
    Go,
    Typescript,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliExportTarget {
    Mermaid,
    Dot,
    Json,
    Svg,
}
