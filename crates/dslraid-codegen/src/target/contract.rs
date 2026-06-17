use super::CodegenTarget;

#[derive(Debug, Clone, Copy)]
pub struct CodegenContract {
    pub role: &'static str,
    pub input: &'static str,
    pub lossy: bool,
    pub contract: &'static str,
}

impl CodegenContract {
    pub fn lossy_label(self) -> &'static str {
        if self.lossy {
            "yes"
        } else {
            "no"
        }
    }
}

pub(super) fn for_target(target: CodegenTarget) -> CodegenContract {
    match target {
        CodegenTarget::Rust => generated("generated runtime source", rust_contract()),
        CodegenTarget::Go => generated("generated runtime source", go_contract()),
        CodegenTarget::TypeScript => generated("viewer/runtime adapter source", ts_contract()),
        CodegenTarget::Mermaid => lossy("documentation diagram", mermaid_contract()),
        CodegenTarget::Dot => lossy("dense graph layout input", dot_contract()),
    }
}

fn generated(role: &'static str, contract: &'static str) -> CodegenContract {
    CodegenContract {
        role,
        input: "Canonical IR",
        lossy: false,
        contract,
    }
}

fn lossy(role: &'static str, contract: &'static str) -> CodegenContract {
    CodegenContract {
        role,
        input: "Canonical IR",
        lossy: true,
        contract,
    }
}

fn rust_contract() -> &'static str {
    "Rust can be runtime code or generated output; Lisp forms stay SSOT."
}

fn go_contract() -> &'static str {
    "Go output follows the same modeled FSM behavior contract."
}

fn ts_contract() -> &'static str {
    "TypeScript output is derived tooling surface, not authoring truth."
}

fn mermaid_contract() -> &'static str {
    "Mermaid export is for readable docs and may omit executable detail."
}

fn dot_contract() -> &'static str {
    "DOT export is for layout/debugging and may omit architecture metadata."
}
