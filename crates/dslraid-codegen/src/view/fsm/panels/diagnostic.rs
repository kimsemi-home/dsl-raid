use dslraid_core::CoreIr;

use crate::view::{InspectorRow, InspectorSection};

pub(crate) fn diagnostic_section(ir: &CoreIr, subject: &str) -> Option<InspectorSection> {
    let rows: Vec<InspectorRow> = ir
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.subjects.iter().any(|item| item == subject))
        .flat_map(diagnostic_rows)
        .collect();
    if rows.is_empty() {
        None
    } else {
        Some(InspectorSection {
            title: "Diagnostics".to_string(),
            rows,
        })
    }
}

fn diagnostic_rows(diagnostic: &dslraid_core::CoreDiagnostic) -> Vec<InspectorRow> {
    let mut rows = vec![row(
        &diagnostic.code,
        format!("{}: {}", diagnostic.severity, diagnostic.message),
        Some(diagnostic.id.clone()),
    )];
    if let Some(suggestion) = &diagnostic.suggestion {
        rows.push(row("Suggestion", suggestion.clone(), None));
    }
    rows
}

fn row(label: &str, value: String, subject: Option<String>) -> InspectorRow {
    InspectorRow {
        label: label.to_string(),
        value,
        subject,
    }
}
