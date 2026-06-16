use crate::{AssertionResult, CountSummary, PropositionResult};

pub(crate) fn propositions(results: &[PropositionResult]) -> CountSummary {
    let mut summary = CountSummary::default();
    for result in results {
        status(&mut summary, &result.status);
    }
    summary
}

pub(crate) fn assertions(results: &[AssertionResult]) -> CountSummary {
    let mut summary = CountSummary::default();
    for result in results {
        status(&mut summary, &result.status);
    }
    summary
}

fn status(summary: &mut CountSummary, status: &str) {
    match status {
        "passed" => summary.passed += 1,
        "failed" => summary.failed += 1,
        "warning" => summary.warnings += 1,
        "skipped" => summary.skipped += 1,
        "not_applicable" => summary.not_applicable += 1,
        _ => {}
    }
}
