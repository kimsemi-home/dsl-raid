use dslraid_codegen::CodegenTarget;

pub(super) fn codegen_target(value: crate::CliCodegenTarget) -> CodegenTarget {
    match value {
        crate::CliCodegenTarget::Rust => CodegenTarget::Rust,
        crate::CliCodegenTarget::Go => CodegenTarget::Go,
        crate::CliCodegenTarget::Typescript => CodegenTarget::TypeScript,
    }
}

pub(super) fn export_target(value: crate::CliExportTarget) -> CodegenTarget {
    match value {
        crate::CliExportTarget::Mermaid => CodegenTarget::Mermaid,
        crate::CliExportTarget::Dot => CodegenTarget::Dot,
        crate::CliExportTarget::Json | crate::CliExportTarget::Svg => {
            unreachable!("JSON and SVG exports are handled without codegen")
        }
    }
}
