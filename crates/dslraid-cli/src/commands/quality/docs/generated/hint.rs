use std::path::Path;

pub(super) fn message(input: &Path) -> String {
    format!(
        "refresh generated docs with `dslraid generate {} --cli-doc docs/generated/cli-reference.md`",
        input.display()
    )
}
