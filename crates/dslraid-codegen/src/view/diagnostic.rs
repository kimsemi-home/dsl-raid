use dslraid_core::{CoreDiagnostic, CoreIr};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub(crate) struct DiagnosticMark {
    pub(crate) badge: &'static str,
    pub(crate) tone: &'static str,
    rank: u8,
}

pub(crate) struct DiagnosticMarks {
    marks: HashMap<String, DiagnosticMark>,
}

impl DiagnosticMarks {
    pub(crate) fn from_ir(ir: &CoreIr) -> Self {
        let mut marks = HashMap::new();
        for diagnostic in &ir.diagnostics {
            if let Some(mark) = severity_mark(diagnostic) {
                merge_subjects(&mut marks, diagnostic, mark);
            }
        }
        Self { marks }
    }

    pub(crate) fn badge(&self, subject: &str) -> Option<&'static str> {
        self.marks.get(subject).map(|mark| mark.badge)
    }

    pub(crate) fn tone(&self, subject: &str) -> Option<&'static str> {
        self.marks.get(subject).map(|mark| mark.tone)
    }
}

fn merge_subjects(
    marks: &mut HashMap<String, DiagnosticMark>,
    diagnostic: &CoreDiagnostic,
    mark: DiagnosticMark,
) {
    for subject in &diagnostic.subjects {
        let current = marks.get(subject);
        if current.is_none_or(|existing| mark.rank > existing.rank) {
            marks.insert(subject.clone(), mark);
        }
    }
}

fn severity_mark(diagnostic: &CoreDiagnostic) -> Option<DiagnosticMark> {
    match diagnostic.severity.as_str() {
        "error" => Some(mark("diag:error", "danger", 4)),
        "warning" => Some(mark("diag:warn", "warning", 3)),
        "info" => Some(mark("diag:info", "muted", 2)),
        "hint" => Some(mark("diag:hint", "muted", 1)),
        _ => None,
    }
}

fn mark(badge: &'static str, tone: &'static str, rank: u8) -> DiagnosticMark {
    DiagnosticMark { badge, tone, rank }
}
