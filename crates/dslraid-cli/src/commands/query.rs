use crate::OutputFormat;
use anyhow::{bail, Result};
use dslraid_core::{event_subject, load_core_ir, state_subject, transition_subject, CoreIr};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(crate) fn run(input: &Path, expression: &str, format: OutputFormat) -> Result<()> {
    let ir = load_core_ir(input)?;
    let items = values(&ir, expression)?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&items)?),
        OutputFormat::Text => {
            for item in items {
                println!(
                    "{} {} {}",
                    item.get("kind")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown"),
                    item.get("subject")
                        .and_then(Value::as_str)
                        .unwrap_or("<unknown>"),
                    item.get("label").and_then(Value::as_str).unwrap_or("")
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn values(ir: &CoreIr, expression: &str) -> Result<Vec<Value>> {
    let filters = parse_query(expression)?;
    Ok(build_query_items(ir)
        .into_iter()
        .filter(|item| matches_query(item, &filters))
        .collect())
}

pub(crate) fn item_map(ir: &CoreIr) -> BTreeMap<String, Value> {
    build_query_items(ir)
        .into_iter()
        .filter_map(|item| {
            let subject = item
                .get("subject")
                .and_then(Value::as_str)
                .map(str::to_string)?;
            Some((subject, item))
        })
        .collect()
}

#[derive(Debug, Clone)]
struct QueryExpression {
    groups: Vec<Vec<QueryClause>>,
}

#[derive(Debug, Clone)]
struct QueryClause {
    key: String,
    operator: QueryOperator,
}

#[derive(Debug, Clone)]
enum QueryOperator {
    Eq(String),
    NotEq(String),
    Contains(String),
    Prefix(String),
    Suffix(String),
    GreaterThan(String),
    GreaterOrEqual(String),
    LessThan(String),
    LessOrEqual(String),
    In(Vec<String>),
    Exists,
    Missing,
}

fn parse_query(expression: &str) -> Result<QueryExpression> {
    let expression = expression.trim();
    if expression.is_empty() || expression == "*" {
        return Ok(QueryExpression {
            groups: vec![Vec::new()],
        });
    }
    let groups = split_logical(expression, "or")
        .into_iter()
        .map(|group| {
            split_logical(&group, "and")
                .into_iter()
                .map(|clause| parse_query_clause(&clause))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(QueryExpression { groups })
}

fn parse_query_clause(clause: &str) -> Result<QueryClause> {
    let clause = clause.trim();
    let lower = clause.to_ascii_lowercase();
    if let Some(key) = lower.strip_suffix(" exists") {
        return query_clause(key, QueryOperator::Exists);
    }
    if let Some(key) = lower.strip_suffix(" missing") {
        return query_clause(key, QueryOperator::Missing);
    }
    if let Some((key, value)) = split_word_operator(clause, "in") {
        let values = parse_list_value(value);
        if values.is_empty() {
            bail!("query in-list cannot be empty: {clause}");
        }
        return query_clause(key, QueryOperator::In(values));
    }
    for (operator, factory) in [
        ("!=", QueryOperator::NotEq as fn(String) -> QueryOperator),
        (">=", QueryOperator::GreaterOrEqual),
        ("<=", QueryOperator::LessOrEqual),
        ("~=", QueryOperator::Contains),
        ("^=", QueryOperator::Prefix),
        ("$=", QueryOperator::Suffix),
        (">", QueryOperator::GreaterThan),
        ("<", QueryOperator::LessThan),
        ("=", QueryOperator::Eq),
    ] {
        if let Some((key, value)) = clause.split_once(operator) {
            return query_clause(key, factory(normalize_query_value(value)));
        }
    }
    bail!("query clause must use an operator: {clause}")
}

fn query_clause(key: &str, operator: QueryOperator) -> Result<QueryClause> {
    let key = key.trim();
    if key.is_empty() {
        bail!("query key cannot be empty");
    }
    Ok(QueryClause {
        key: key.to_ascii_lowercase(),
        operator,
    })
}

fn split_word_operator<'a>(clause: &'a str, operator: &str) -> Option<(&'a str, &'a str)> {
    let needle = format!(" {operator} ");
    clause
        .to_ascii_lowercase()
        .find(&needle)
        .map(|index| (&clause[..index], &clause[index + needle.len()..]))
}

fn split_logical(input: &str, operator: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut quote: Option<char> = None;
    let mut bracket_depth = 0usize;
    let bytes = input.as_bytes();
    let needle = format!(" {operator} ");
    let lower = input.to_ascii_lowercase();
    let lower_bytes = lower.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        let character = input[index..].chars().next().unwrap_or_default();
        match character {
            '\'' | '"' if quote == Some(character) => quote = None,
            '\'' | '"' if quote.is_none() => quote = Some(character),
            '[' if quote.is_none() => bracket_depth += 1,
            ']' if quote.is_none() && bracket_depth > 0 => bracket_depth -= 1,
            _ => {}
        }
        if quote.is_none()
            && bracket_depth == 0
            && lower_bytes[index..].starts_with(needle.as_bytes())
        {
            parts.push(input[start..index].trim().to_string());
            index += needle.len();
            start = index;
            continue;
        }
        index += character.len_utf8();
    }
    parts.push(input[start..].trim().to_string());
    parts
}

fn normalize_query_value(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn parse_list_value(value: &str) -> Vec<String> {
    let value = value.trim().trim_start_matches('[').trim_end_matches(']');
    value
        .split(',')
        .map(normalize_query_value)
        .filter(|item| !item.is_empty())
        .collect()
}

fn build_query_items(ir: &CoreIr) -> Vec<Value> {
    let mut items = Vec::new();
    let mut tested_subjects = BTreeSet::new();
    let mut generated_subjects = BTreeSet::new();
    for derivation in &ir.derivations {
        for target in &derivation.targets {
            if target.role == "test" {
                tested_subjects.insert(derivation.source.clone());
            }
            if target.role == "generated" {
                generated_subjects.insert(derivation.source.clone());
                generated_subjects.insert(target.artifact.clone());
            }
        }
    }

    push_query_item(
        &mut items,
        "project",
        &format!("project:{}", ir.project.id),
        &ir.project.id,
        &ir.project.name,
        &ir.project.tags,
        ir.project.visibility.as_deref(),
        None,
        &tested_subjects,
        &generated_subjects,
        None,
    );
    for context in &ir.contexts {
        push_query_item(
            &mut items,
            "context",
            &context.id,
            &context.id,
            &context.name,
            &context.tags,
            context.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            None,
        );
    }
    for requirement in &ir.requirements {
        push_query_item(
            &mut items,
            "requirement",
            &requirement.id,
            &requirement.id,
            &requirement.name,
            &requirement.tags,
            requirement.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            None,
        );
    }
    for capability in &ir.capabilities {
        push_query_item(
            &mut items,
            "capability",
            &capability.id,
            &capability.id,
            &capability.name,
            &capability.tags,
            capability.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "capability_kind": capability.kind,
                "owner": capability.owner
            })),
        );
    }
    for policy in &ir.policies {
        push_query_item(
            &mut items,
            "policy",
            &policy.id,
            &policy.id,
            &policy.name,
            &policy.tags,
            policy.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "policy_kind": policy.kind,
                "applies_to": policy.applies_to
            })),
        );
    }
    for command in &ir.commands {
        push_query_item(
            &mut items,
            "command",
            &command.id,
            &command.id,
            &command.name,
            &command.tags,
            command.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "capability": command.capability
            })),
        );
    }
    for fsm in &ir.fsms {
        push_query_item(
            &mut items,
            "fsm",
            &fsm.id,
            &fsm.id,
            &fsm.name,
            &fsm.tags,
            fsm.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "context": fsm.context,
                "states": fsm.states.len(),
                "transitions": fsm.transitions.len()
            })),
        );
        for state in &fsm.states {
            let subject = state_subject(&fsm.id, &state.id);
            push_query_item(
                &mut items,
                "state",
                &subject,
                &state.id,
                &state.id,
                &state.tags,
                state.visibility.as_deref(),
                None,
                &tested_subjects,
                &generated_subjects,
                Some(serde_json::json!({
                    "fsm": fsm.id,
                    "state_kind": state.kind,
                    "initial": state.initial,
                    "terminal": state.terminal,
                    "terminal_semantics": state.terminal_semantics
                })),
            );
        }
        for event in &fsm.events {
            let subject = event_subject(&fsm.id, &event.id);
            push_query_item(
                &mut items,
                "event",
                &subject,
                &event.id,
                event.name.as_deref().unwrap_or(&event.id),
                &event.tags,
                event.visibility.as_deref(),
                None,
                &tested_subjects,
                &generated_subjects,
                Some(serde_json::json!({
                    "fsm": fsm.id,
                    "event_kind": event.kind
                })),
            );
        }
        for guard in &fsm.guards {
            let subject = format!("guard:{}.{}", fsm.local_name(), guard.id);
            push_query_item(
                &mut items,
                "guard",
                &subject,
                &guard.id,
                guard.name.as_deref().unwrap_or(&guard.id),
                &guard.tags,
                guard.visibility.as_deref(),
                None,
                &tested_subjects,
                &generated_subjects,
                Some(serde_json::json!({
                    "fsm": fsm.id,
                    "capability": guard.capability
                })),
            );
        }
        for action in &fsm.actions {
            let subject = format!("action:{}.{}", fsm.local_name(), action.id);
            push_query_item(
                &mut items,
                "action",
                &subject,
                &action.id,
                action.name.as_deref().unwrap_or(&action.id),
                &action.tags,
                action.visibility.as_deref(),
                None,
                &tested_subjects,
                &generated_subjects,
                Some(serde_json::json!({
                    "fsm": fsm.id,
                    "capability": action.capability,
                    "depends_on": action.depends_on
                })),
            );
        }
        for transition in &fsm.transitions {
            let subject = transition_subject(&fsm.id, &transition.id);
            push_query_item(
                &mut items,
                "transition",
                &subject,
                &transition.id,
                &transition.id,
                &transition.tags,
                transition.visibility.as_deref(),
                None,
                &tested_subjects,
                &generated_subjects,
                Some(serde_json::json!({
                    "fsm": fsm.id,
                    "from": state_subject(&fsm.id, &transition.from),
                    "to": state_subject(&fsm.id, &transition.to),
                    "on": transition.on.as_ref().map(|event| event_subject(&fsm.id, event)),
                    "guards": transition.guards,
                    "actions": transition.actions,
                    "requires": transition.requires
                })),
            );
        }
    }
    for composition in &ir.compositions {
        push_query_item(
            &mut items,
            "composition",
            &composition.id,
            &composition.id,
            &composition.name,
            &composition.tags,
            composition.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "composition_kind": composition.kind,
                "inputs": composition.inputs
            })),
        );
    }
    for projection in &ir.projections {
        push_query_item(
            &mut items,
            "projection",
            &projection.id,
            &projection.id,
            &projection.id,
            &projection.tags,
            projection.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "source": projection.source,
                "show": projection.show
            })),
        );
    }
    for derivation in &ir.derivations {
        push_query_item(
            &mut items,
            "derivation",
            &derivation.id,
            &derivation.id,
            &derivation.id,
            &derivation.tags,
            derivation.visibility.as_deref(),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "source": derivation.source,
                "rule": derivation.rule.id,
                "targets": derivation.targets.iter().map(|target| &target.artifact).collect::<Vec<_>>()
            })),
        );
    }
    for artifact in &ir.artifacts {
        push_query_item(
            &mut items,
            "artifact",
            &artifact.id,
            &artifact.id,
            &artifact.path,
            &artifact.tags,
            artifact.visibility.as_deref(),
            Some(&artifact.path),
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "artifact_kind": artifact.kind,
                "generated_by": artifact.generated_by
            })),
        );
    }
    for diagnostic in &ir.diagnostics {
        push_query_item(
            &mut items,
            "diagnostic",
            &diagnostic.id,
            &diagnostic.id,
            &diagnostic.message,
            &[],
            Some(&diagnostic.severity),
            None,
            &tested_subjects,
            &generated_subjects,
            Some(serde_json::json!({
                "code": diagnostic.code,
                "severity": diagnostic.severity,
                "subjects": diagnostic.subjects
            })),
        );
    }
    items
}

#[allow(clippy::too_many_arguments)]
fn push_query_item(
    items: &mut Vec<Value>,
    kind: &str,
    subject: &str,
    id: &str,
    label: &str,
    tags: &[String],
    visibility: Option<&str>,
    path: Option<&str>,
    tested_subjects: &BTreeSet<String>,
    generated_subjects: &BTreeSet<String>,
    extra: Option<Value>,
) {
    let tag_set: BTreeSet<&str> = tags.iter().map(String::as_str).collect();
    let mut item = serde_json::json!({
        "kind": kind,
        "subject": subject,
        "id": id,
        "label": label,
        "visibility": visibility,
        "tags": tags,
        "path": path,
        "tested": tag_set.contains("tested") || tested_subjects.contains(subject),
        "generated": tag_set.contains("generated") || generated_subjects.contains(subject)
    });
    if let Some(extra) = extra.and_then(|value| value.as_object().cloned()) {
        let object = item.as_object_mut().expect("query item is an object");
        for (key, value) in extra {
            object.insert(key, value);
        }
    }
    items.push(item);
}

fn matches_query(item: &Value, expression: &QueryExpression) -> bool {
    expression.groups.iter().any(|group| {
        group.iter().all(|clause| {
            if clause.key == "tag" {
                return item
                    .get("tags")
                    .and_then(Value::as_array)
                    .is_some_and(|tags| {
                        tags.iter()
                            .any(|tag| value_matches_operator(Some(tag), &clause.operator))
                    });
            }
            value_matches_operator(query_value(item, &clause.key), &clause.operator)
        })
    })
}

fn query_value<'a>(item: &'a Value, key: &str) -> Option<&'a Value> {
    let mut current = item;
    for part in key.split('.') {
        current = current.get(part)?;
    }
    Some(current)
}

fn value_matches_operator(actual: Option<&Value>, operator: &QueryOperator) -> bool {
    match operator {
        QueryOperator::Exists => actual.is_some_and(value_exists),
        QueryOperator::Missing => actual.is_none_or(|value| !value_exists(value)),
        QueryOperator::Eq(expected) => actual.is_some_and(|value| value_matches(value, expected)),
        QueryOperator::NotEq(expected) => {
            actual.is_some_and(|value| !value_matches(value, expected))
        }
        QueryOperator::Contains(expected) => {
            actual.is_some_and(|value| value_contains(value, expected))
        }
        QueryOperator::Prefix(expected) => actual.is_some_and(|value| {
            value.as_str().is_some_and(|actual| {
                actual
                    .to_ascii_lowercase()
                    .starts_with(&expected.to_ascii_lowercase())
            })
        }),
        QueryOperator::Suffix(expected) => actual.is_some_and(|value| {
            value.as_str().is_some_and(|actual| {
                actual
                    .to_ascii_lowercase()
                    .ends_with(&expected.to_ascii_lowercase())
            })
        }),
        QueryOperator::GreaterThan(expected) => {
            compare_numbers(actual, expected, |left, right| left > right)
        }
        QueryOperator::GreaterOrEqual(expected) => {
            compare_numbers(actual, expected, |left, right| left >= right)
        }
        QueryOperator::LessThan(expected) => {
            compare_numbers(actual, expected, |left, right| left < right)
        }
        QueryOperator::LessOrEqual(expected) => {
            compare_numbers(actual, expected, |left, right| left <= right)
        }
        QueryOperator::In(expected) => actual.is_some_and(|value| {
            expected
                .iter()
                .any(|expected| value_matches(value, expected))
        }),
    }
}

fn value_exists(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Array(values) => !values.is_empty(),
        Value::String(value) => !value.is_empty(),
        _ => true,
    }
}

fn compare_numbers(
    actual: Option<&Value>,
    expected: &str,
    predicate: impl Fn(f64, f64) -> bool,
) -> bool {
    let Some(actual) = actual.and_then(value_as_f64) else {
        return false;
    };
    let Ok(expected) = expected.parse::<f64>() else {
        return false;
    };
    predicate(actual, expected)
}

fn value_as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(value) => value.parse().ok(),
        _ => None,
    }
}

fn value_contains(actual: &Value, expected: &str) -> bool {
    match actual {
        Value::String(value) => value
            .to_ascii_lowercase()
            .contains(&expected.to_ascii_lowercase()),
        Value::Array(values) => values.iter().any(|value| value_contains(value, expected)),
        Value::Bool(_) | Value::Number(_) | Value::Null => value_matches(actual, expected),
        Value::Object(object) => object.values().any(|value| value_contains(value, expected)),
    }
}

fn value_matches(actual: &Value, expected: &str) -> bool {
    match actual {
        Value::Bool(value) => match expected.to_ascii_lowercase().as_str() {
            "true" | "yes" | "1" => *value,
            "false" | "no" | "0" => !*value,
            _ => false,
        },
        Value::String(value) => value.eq_ignore_ascii_case(expected),
        Value::Number(value) => value.to_string() == expected,
        Value::Array(values) => values.iter().any(|value| value_matches(value, expected)),
        Value::Null => expected.eq_ignore_ascii_case("null"),
        Value::Object(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn values_support_policy_or_terminal_query() {
        let ir = load_core_ir(runscope_fixture()).unwrap();

        let results = values(
            &ir,
            "kind=transition and requires~=policy:no_secret_leak or terminal=true",
        )
        .unwrap();

        assert!(results.iter().any(|item| {
            item.get("subject").and_then(Value::as_str)
                == Some("transition:runtime.running_to_completed")
        }));
        assert!(results.iter().any(|item| {
            item.get("subject").and_then(Value::as_str) == Some("state:runtime.completed")
        }));
    }

    #[test]
    fn values_support_numeric_query() {
        let ir = load_core_ir(runscope_fixture()).unwrap();

        let results = values(&ir, "kind=fsm and states>=1").unwrap();

        assert!(results
            .iter()
            .any(|item| item.get("subject").and_then(Value::as_str) == Some("fsm:runtime")));
    }

    #[test]
    fn item_map_exposes_transition_endpoints() {
        let ir = load_core_ir(runscope_fixture()).unwrap();
        let items = item_map(&ir);

        let transition = items
            .get("transition:runtime.idle_to_starting")
            .expect("fixture transition is indexed");

        assert_eq!(
            transition.get("from").and_then(Value::as_str),
            Some("state:runtime.idle")
        );
        assert_eq!(
            transition.get("to").and_then(Value::as_str),
            Some("state:runtime.starting")
        );
    }

    fn runscope_fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/runscope.raid.json")
    }
}
