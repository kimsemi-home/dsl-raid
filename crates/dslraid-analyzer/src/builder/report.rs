use crate::builder::{count, diagnostic, result, source, AssertionSpec};
use crate::{
    AssertionResult, DiagnosticRef, PropositionResult, Summary, ValidateOptions, ValidationReport,
    VALIDATION_VERSION,
};

pub(crate) struct ReportBuilder {
    options: ValidateOptions,
    propositions: Vec<PropositionResult>,
    assertions: Vec<AssertionResult>,
    diagnostics: Vec<DiagnosticRef>,
}

impl ReportBuilder {
    pub(crate) fn new(options: ValidateOptions) -> Self {
        Self {
            options,
            propositions: Vec::new(),
            assertions: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn record(&mut self, spec: AssertionSpec) {
        let diagnostics = diagnostic::diagnostic_for(&spec);
        self.diagnostics.extend(diagnostics.clone());
        self.propositions.push(PropositionResult {
            id: spec.proposition.to_string(),
            layer: spec.layer.to_string(),
            status: spec.status.to_string(),
            severity: spec.severity.to_string(),
            subjects: spec.subjects.clone(),
            diagnostics: diagnostics.clone(),
            assertions: vec![spec.assertion.to_string()],
            message: spec.message.clone(),
        });
        self.assertions.push(result::assertion(spec, diagnostics));
    }

    pub(crate) fn finish(self) -> ValidationReport {
        let failed = self.assertions.iter().any(result::is_blocking);
        ValidationReport {
            validation_version: VALIDATION_VERSION.to_string(),
            source: source::source(self.options.source_path, self.options.ir_hash),
            run: source::run(self.options.mode, self.options.deny),
            summary: Summary {
                status: if failed { "failed" } else { "passed" }.to_string(),
                propositions: count::propositions(&self.propositions),
                assertions: count::assertions(&self.assertions),
            },
            propositions: self.propositions,
            assertions: self.assertions,
            diagnostics: self.diagnostics,
        }
    }
}
