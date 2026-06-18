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
