use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dslraid_analyzer::{validate_core_ir, ValidateOptions, ValidationReport};
use dslraid_codegen::{generate_code, project_view, render_svg, CodegenTarget};
use dslraid_core::{
    event_subject, load_core_ir, sha256_json, state_subject, transition_subject,
    validate_json_schema, CORE_SCHEMA_PATH, VALIDATION_SCHEMA_PATH, VIEW_SCHEMA_PATH,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod commands;

#[derive(Debug, Parser)]
#[command(
    name = "dslraid",
    version,
    about = "Executable architecture IR browser CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init {
        #[arg(default_value = ".dslraid.json")]
        out: PathBuf,
    },
    Normalize {
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Migrate {
        input: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Validate {
        input: PathBuf,
        #[arg(long, default_value = CORE_SCHEMA_PATH)]
        schema: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long = "deny")]
        deny: Vec<String>,
    },
    Schema {
        #[command(subcommand)]
        command: SchemaCommand,
    },
    Quality,
    Golden {
        #[command(subcommand)]
        command: GoldenCommand,
    },
    Project {
        input: PathBuf,
        #[arg(long)]
        projection: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Render {
        input: PathBuf,
        #[arg(long)]
        projection: Option<String>,
        #[arg(long, value_enum, default_value_t = RenderFormat::Svg)]
        format: RenderFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Codegen {
        input: PathBuf,
        #[arg(long, value_enum)]
        target: CliCodegenTarget,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Compose {
        input: PathBuf,
        #[arg(long)]
        composition: Option<String>,
        #[arg(long, default_value = "diagnostics-only")]
        materialize: String,
        #[arg(long, default_value_t = 5000)]
        limit: usize,
        #[arg(long)]
        focus: Option<String>,
        #[arg(long, default_value_t = 1)]
        depth: usize,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Diff {
        base: PathBuf,
        head: PathBuf,
        #[arg(long, value_enum, default_value_t = DiffFormat::Text)]
        format: DiffFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Query {
        input: PathBuf,
        expression: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    Trace {
        #[command(subcommand)]
        command: TraceCommand,
    },
    Coverage {
        #[command(subcommand)]
        command: CoverageCommand,
    },
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
    Compat {
        #[command(subcommand)]
        command: CompatCommand,
    },
    Export {
        #[arg(value_enum)]
        target: CliExportTarget,
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum SchemaCommand {
    Validate { schema: PathBuf, input: PathBuf },
}

#[derive(Debug, Subcommand)]
enum GoldenCommand {
    Check { path: PathBuf },
    Update { path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum TraceCommand {
    Import {
        input: PathBuf,
        #[arg(long)]
        design_ir: Option<PathBuf>,
        #[arg(long)]
        run_id: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        trace: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
enum CoverageCommand {
    Build {
        #[arg(long)]
        trace: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        coverage: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
    Verify {
        input: PathBuf,
        #[arg(long)]
        lock: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
enum CompatCommand {
    Check { input: PathBuf },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DiffFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RenderFormat {
    Svg,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliCodegenTarget {
    Rust,
    Go,
    Typescript,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliExportTarget {
    Mermaid,
    Dot,
    Json,
    Svg,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { out } => init_project(&out),
        Command::Normalize { input, out } => normalize(&input, out.as_deref()),
        Command::Migrate {
            input,
            from,
            to,
            out,
        } => migrate(&input, &from, &to, out.as_deref()),
        Command::Validate {
            input,
            schema,
            format,
            deny,
        } => validate(&input, &schema, format, deny),
        Command::Schema { command } => match command {
            SchemaCommand::Validate { schema, input } => schema_validate(&schema, &input),
        },
        Command::Quality => quality(),
        Command::Golden { command } => match command {
            GoldenCommand::Check { path } => golden_check(&path),
            GoldenCommand::Update { path } => golden_update(&path),
        },
        Command::Project {
            input,
            projection,
            out,
        } => project(&input, projection.as_deref(), out.as_deref()),
        Command::Render {
            input,
            projection,
            format,
            out,
        } => render(&input, projection.as_deref(), format, out.as_deref()),
        Command::Codegen { input, target, out } => codegen(&input, target, out.as_deref()),
        Command::Compose {
            input,
            composition,
            materialize,
            limit,
            focus,
            depth,
            format,
            out,
        } => compose(
            &input,
            composition.as_deref(),
            &materialize,
            limit,
            focus.as_deref(),
            depth,
            format,
            out.as_deref(),
        ),
        Command::Diff {
            base,
            head,
            format,
            out,
        } => diff(&base, &head, format, out.as_deref()),
        Command::Query {
            input,
            expression,
            format,
        } => query(&input, &expression, format),
        Command::Trace { command } => match command {
            TraceCommand::Import {
                input,
                design_ir,
                run_id,
                out,
            } => trace_import(
                &input,
                design_ir.as_deref(),
                run_id.as_deref(),
                out.as_deref(),
            ),
            TraceCommand::Check {
                trace,
                design_ir,
                format,
            } => trace_check(&trace, &design_ir, format),
        },
        Command::Coverage { command } => match command {
            CoverageCommand::Build {
                trace,
                design_ir,
                out,
            } => coverage_build(&trace, &design_ir, out.as_deref()),
            CoverageCommand::Check {
                coverage,
                design_ir,
                format,
            } => coverage_check(&coverage, &design_ir, format),
        },
        Command::Artifact { command } => match command {
            ArtifactCommand::Verify {
                input,
                lock,
                format,
            } => commands::artifact::verify(&input, lock.as_deref(), format),
        },
        Command::Compat { command } => match command {
            CompatCommand::Check { input } => compat_check(&input),
        },
        Command::Export { target, input, out } => export(&input, target, out.as_deref()),
    }
}

fn init_project(out: &Path) -> Result<()> {
    if out.exists() {
        bail!("{} already exists", out.display());
    }
    let template = serde_json::json!({
        "ir_version": "0.1.0",
        "project": {
            "id": "dslraid-project",
            "name": "DSLRaid Project",
            "visibility": "public"
        },
        "contexts": [],
        "requirements": [],
        "capabilities": [],
        "policies": [],
        "commands": [],
        "fsms": [],
        "compositions": [],
        "projections": [],
        "derivations": [],
        "artifacts": [],
        "diagnostics": []
    });
    write_bytes(out, serde_json::to_string_pretty(&template)?.as_bytes())?;
    println!("created {}", out.display());
    Ok(())
}

fn normalize(input: &Path, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let bytes = serde_json::to_vec_pretty(&ir)?;
    write_or_stdout(out, &bytes)
}

fn migrate(input: &Path, from: &str, to: &str, out: Option<&Path>) -> Result<()> {
    let mut ir = load_core_ir(input)?;
    if ir.ir_version != from {
        bail!(
            "input IR version is {}, but --from was {}",
            ir.ir_version,
            from
        );
    }
    if from != to {
        bail!("no migration rule registered for {from} -> {to}");
    }
    ir.ir_version = to.to_string();
    let bytes = serde_json::to_vec_pretty(&ir)?;
    write_or_stdout(out, &bytes)
}

fn validate(input: &Path, schema: &Path, format: OutputFormat, deny: Vec<String>) -> Result<()> {
    let schema_issues = validate_json_schema(schema, input)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => {
                for issue in &schema_issues {
                    println!("schema error at {}: {}", issue.instance_path, issue.message);
                }
            }
        }
        bail!(
            "schema validation failed with {} issues",
            schema_issues.len()
        );
    }

    let ir = load_core_ir(input)?;
    let report = validation_report(&ir, input, "validate", deny.clone())?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_report_text(&report),
    }
    if !report.is_success(&deny) {
        bail!("validation failed");
    }
    Ok(())
}

fn validation_report(
    ir: &dslraid_core::CoreIr,
    input: &Path,
    mode: &str,
    deny: Vec<String>,
) -> Result<ValidationReport> {
    let hash = sha256_json(ir)?;
    Ok(validate_core_ir(
        ir,
        ValidateOptions {
            source_path: input.display().to_string(),
            ir_hash: Some(hash),
            mode: mode.to_string(),
            deny,
        },
    ))
}

fn schema_validate(schema: &Path, input: &Path) -> Result<()> {
    let issues = validate_json_schema(schema, input)?;
    if issues.is_empty() {
        println!("schema ok: {}", input.display());
        Ok(())
    } else {
        for issue in &issues {
            println!("schema error at {}: {}", issue.instance_path, issue.message);
        }
        bail!("schema validation failed with {} issues", issues.len())
    }
}

fn validate_json_file(schema: &Path, input: &Path) -> Result<()> {
    let issues = validate_json_schema(schema, input)?;
    if issues.is_empty() {
        Ok(())
    } else {
        bail!(
            "{} failed schema validation with {} issues",
            input.display(),
            issues.len()
        )
    }
}

fn quality() -> Result<()> {
    check_json_syntax("schemas")?;
    check_json_syntax("examples")?;
    schema_validate(
        Path::new(CORE_SCHEMA_PATH),
        Path::new("examples/runscope/runscope.raid.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-assertion.schema.json"),
        Path::new("examples/runscope/runscope.assertions.json"),
    )?;
    schema_validate(
        Path::new(VALIDATION_SCHEMA_PATH),
        Path::new("examples/runscope/runscope.validation.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-lock.schema.json"),
        Path::new("examples/runscope/runscope.lock.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-annotation.schema.json"),
        Path::new("examples/runscope/runscope.annotations.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-sourcemap.schema.json"),
        Path::new("examples/runscope/runscope.sourcemap.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-trace.schema.json"),
        Path::new("examples/runscope/run-001.trace.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-coverage.schema.json"),
        Path::new("examples/runscope/run-001.coverage.json"),
    )?;

    let input = Path::new("examples/runscope/runscope.raid.json");
    let ir = load_core_ir(input)?;
    let report = validation_report(&ir, input, "quality", Vec::new())?;
    if !report.is_success(&[]) {
        print_report_text(&report);
        bail!("semantic quality failed");
    }
    let view = project_view(&ir, Some("view:runtime"), input.display().to_string())?;
    let view_path = std::env::temp_dir().join(format!("dslraid-view-{}.json", std::process::id()));
    write_bytes(&view_path, serde_json::to_string_pretty(&view)?.as_bytes())?;
    schema_validate(Path::new(VIEW_SCHEMA_PATH), &view_path)?;
    fs::remove_file(&view_path).ok();

    let svg = render_svg(&view);
    if !svg.contains("<svg") || svg.len() < 200 {
        bail!("rendered SVG is unexpectedly empty");
    }
    for target in [
        CliCodegenTarget::Rust,
        CliCodegenTarget::Go,
        CliCodegenTarget::Typescript,
    ] {
        let generated = generate_code(&ir, target.into())?;
        if generated.trim().is_empty() {
            bail!("empty codegen output for {target:?}");
        }
    }
    let transition_query = query_values(&ir, "kind=transition")?;
    if transition_query.is_empty() {
        bail!("query returned no transitions");
    }
    let richer_query = query_values(
        &ir,
        "kind=transition and requires~=policy:no_secret_leak or terminal=true",
    )?;
    if richer_query.is_empty() {
        bail!("richer query returned no results");
    }
    let composition = compose_result(&ir, None, "reachable", 100, None, 1)?;
    if composition
        .get("composition")
        .and_then(|value| value.get("state_space"))
        .and_then(Value::as_u64)
        .unwrap_or_default()
        == 0
    {
        bail!("lazy composition did not compute a state space");
    }
    trace_check(
        Path::new("examples/runscope/run-001.trace.json"),
        input,
        OutputFormat::Text,
    )?;
    let coverage_path =
        std::env::temp_dir().join(format!("dslraid-coverage-{}.json", std::process::id()));
    coverage_build(
        Path::new("examples/runscope/run-001.trace.json"),
        input,
        Some(&coverage_path),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-coverage.schema.json"),
        &coverage_path,
    )?;
    coverage_check(&coverage_path, input, OutputFormat::Text)?;
    fs::remove_file(&coverage_path).ok();
    let imported_trace = std::env::temp_dir().join(format!(
        "dslraid-imported-trace-{}.json",
        std::process::id()
    ));
    trace_import(
        Path::new("examples/runscope/run-002.trace.jsonl"),
        Some(input),
        Some("run-002"),
        Some(&imported_trace),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-trace.schema.json"),
        &imported_trace,
    )?;
    fs::remove_file(&imported_trace).ok();
    let self_diff = diff_report(&ir, &ir, input, input)?;
    if self_diff.status != "unchanged" {
        bail!("self diff should be unchanged");
    }
    commands::artifact::verify(input, None, OutputFormat::Text)?;
    println!("quality ok");
    Ok(())
}

fn trace_import(
    input: &Path,
    design_ir: Option<&Path>,
    run_id: Option<&str>,
    out: Option<&Path>,
) -> Result<()> {
    let source = fs::read_to_string(input).with_context(|| format!("read {}", input.display()))?;
    let trimmed = source.trim_start();
    let is_jsonl = input.extension().and_then(|ext| ext.to_str()) == Some("jsonl");
    let mut trace = if !is_jsonl && trimmed.starts_with('{') {
        serde_json::from_str::<Value>(&source)
            .with_context(|| format!("parse {}", input.display()))?
    } else {
        let mut events = Vec::new();
        for (index, line) in source.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut event: Value = serde_json::from_str(line)
                .with_context(|| format!("parse JSONL line {}", index + 1))?;
            let object = event
                .as_object_mut()
                .ok_or_else(|| anyhow!("JSONL line {} must be an object", index + 1))?;
            object
                .entry("id".to_string())
                .or_insert_with(|| Value::String(format!("evt-{:04}", index + 1)));
            if !object.contains_key("timestamp") {
                bail!("JSONL line {} is missing timestamp", index + 1);
            }
            if !object.contains_key("kind") {
                bail!("JSONL line {} is missing kind", index + 1);
            }
            events.push(event);
        }
        serde_json::json!({
            "trace_version": "0.1.0",
            "run": {
                "id": run_id
                    .map(str::to_string)
                    .unwrap_or_else(|| input.file_stem().and_then(|stem| stem.to_str()).unwrap_or("imported-run").to_string()),
                "environment": "imported"
            },
            "events": events
        })
    };

    if let Some(design_ir) = design_ir {
        let hash = sha256_json(&load_core_ir(design_ir)?)?;
        trace
            .as_object_mut()
            .ok_or_else(|| anyhow!("trace root must be an object"))?
            .insert(
                "design_ir".to_string(),
                serde_json::json!({
                    "path": design_ir.display().to_string(),
                    "hash": hash
                }),
            );
    }

    let temp_path =
        std::env::temp_dir().join(format!("dslraid-trace-import-{}.json", std::process::id()));
    write_bytes(&temp_path, serde_json::to_string_pretty(&trace)?.as_bytes())?;
    let issues = validate_json_schema(Path::new("schemas/dslraid-trace.schema.json"), &temp_path)?;
    fs::remove_file(&temp_path).ok();
    if !issues.is_empty() {
        for issue in &issues {
            println!("schema error at {}: {}", issue.instance_path, issue.message);
        }
        bail!("imported trace failed schema validation");
    }

    write_or_stdout(out, serde_json::to_string_pretty(&trace)?.as_bytes())
}

fn trace_check(trace: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => {
                for issue in &schema_issues {
                    println!("schema error at {}: {}", issue.instance_path, issue.message);
                }
            }
        }
        bail!("trace schema validation failed");
    }
    if matches!(format, OutputFormat::Text) {
        println!("schema ok: {}", trace.display());
    }
    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let known_subjects = ir.semantic_subjects();
    let mut issues = Vec::new();

    for event in trace_value
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("trace.events must be an array"))?
    {
        let event_id = event
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>");
        let kind = event
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>");
        for field in ["subject", "from", "to"] {
            if let Some(subject) = event.get(field).and_then(Value::as_str) {
                if !known_subjects.contains(subject) {
                    issues.push(serde_json::json!({
                        "code": "RTE049",
                        "event": event_id,
                        "field": field,
                        "subject": subject,
                        "message": "runtime trace event does not map to a known design subject"
                    }));
                }
            }
        }
        if matches!(
            kind,
            "transition_started" | "transition_completed" | "transition_failed"
        ) {
            let Some(subject) = event.get("subject").and_then(Value::as_str) else {
                issues.push(serde_json::json!({
                    "code": "RTE049",
                    "event": event_id,
                    "message": "transition trace event is missing subject"
                }));
                continue;
            };
            if let Some((from, to)) = transition_endpoints(&ir, subject) {
                if event
                    .get("from")
                    .and_then(Value::as_str)
                    .is_some_and(|value| value != from)
                {
                    issues.push(serde_json::json!({
                        "code": "RTE050",
                        "event": event_id,
                        "subject": subject,
                        "message": "trace from-state contradicts transition definition",
                        "expected": from,
                        "actual": event.get("from")
                    }));
                }
                if event
                    .get("to")
                    .and_then(Value::as_str)
                    .is_some_and(|value| value != to)
                {
                    issues.push(serde_json::json!({
                        "code": "RTE050",
                        "event": event_id,
                        "subject": subject,
                        "message": "trace to-state contradicts transition definition",
                        "expected": to,
                        "actual": event.get("to")
                    }));
                }
            }
        }
    }

    let report = serde_json::json!({
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "trace": trace.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => {
            if report.get("status").and_then(Value::as_str) == Some("passed") {
                println!("trace check passed");
            } else {
                println!("trace check failed");
                if let Some(issues) = report.get("issues").and_then(Value::as_array) {
                    for issue in issues {
                        println!(
                            "{} {}: {}",
                            issue
                                .get("code")
                                .and_then(Value::as_str)
                                .unwrap_or("RTE000"),
                            issue
                                .get("event")
                                .and_then(Value::as_str)
                                .unwrap_or("<unknown>"),
                            issue.get("message").and_then(Value::as_str).unwrap_or("")
                        );
                    }
                }
            }
        }
    }

    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("trace check failed")
    }
}

fn coverage_build(trace: &Path, design_ir: &Path, out: Option<&Path>) -> Result<()> {
    validate_json_file(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let coverage = coverage_overlay_value(&ir, design_ir, trace, &trace_value)?;
    let temp_path = std::env::temp_dir().join(format!(
        "dslraid-coverage-build-{}.json",
        std::process::id()
    ));
    write_bytes(
        &temp_path,
        serde_json::to_string_pretty(&coverage)?.as_bytes(),
    )?;
    validate_json_file(
        Path::new("schemas/dslraid-coverage.schema.json"),
        &temp_path,
    )?;
    fs::remove_file(&temp_path).ok();
    write_or_stdout(out, serde_json::to_string_pretty(&coverage)?.as_bytes())
}

fn coverage_check(coverage: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-coverage.schema.json"), coverage)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => {
                for issue in &schema_issues {
                    println!("schema error at {}: {}", issue.instance_path, issue.message);
                }
            }
        }
        bail!("coverage schema validation failed");
    }
    let ir = load_core_ir(design_ir)?;
    let coverage_value: Value = serde_json::from_slice(&fs::read(coverage)?)?;
    let known_subjects = ir.semantic_subjects();
    let mut issues = Vec::new();
    let mut covered_subjects = BTreeSet::new();
    for subject in coverage_value
        .get("subjects")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("coverage.subjects must be an array"))?
    {
        let Some(subject_id) = subject.get("subject").and_then(Value::as_str) else {
            continue;
        };
        if !known_subjects.contains(subject_id) {
            issues.push(serde_json::json!({
                "code": "COV001",
                "subject": subject_id,
                "message": "Coverage subject does not resolve to the design IR."
            }));
        }
        covered_subjects.insert(subject_id.to_string());
    }
    for fsm in &ir.fsms {
        for state in &fsm.states {
            let subject = state_subject(&fsm.id, &state.id);
            if !covered_subjects.contains(&subject) {
                issues.push(serde_json::json!({
                    "code": "COV002",
                    "subject": subject,
                    "message": "Coverage overlay is missing a state subject."
                }));
            }
        }
        for transition in &fsm.transitions {
            let subject = transition_subject(&fsm.id, &transition.id);
            if !covered_subjects.contains(&subject) {
                issues.push(serde_json::json!({
                    "code": "COV002",
                    "subject": subject,
                    "message": "Coverage overlay is missing a transition subject."
                }));
            }
        }
    }
    let report = serde_json::json!({
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "coverage": coverage.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => {
            if report.get("status").and_then(Value::as_str) == Some("passed") {
                println!("coverage check passed");
            } else {
                println!("coverage check failed");
                if let Some(issues) = report.get("issues").and_then(Value::as_array) {
                    for issue in issues {
                        println!(
                            "{} {}: {}",
                            issue
                                .get("code")
                                .and_then(Value::as_str)
                                .unwrap_or("COV000"),
                            issue
                                .get("subject")
                                .and_then(Value::as_str)
                                .unwrap_or("<unknown>"),
                            issue.get("message").and_then(Value::as_str).unwrap_or("")
                        );
                    }
                }
            }
        }
    }
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("coverage check failed")
    }
}

#[derive(Debug, Clone)]
struct CoverageCounter {
    kind: String,
    count: usize,
    failures: usize,
    status_override: Option<String>,
    last_seen: Option<String>,
}

fn coverage_overlay_value(
    ir: &dslraid_core::CoreIr,
    design_ir: &Path,
    trace: &Path,
    trace_value: &Value,
) -> Result<Value> {
    let mut counters = base_coverage_counters(ir);
    for event in trace_value
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("trace.events must be an array"))?
    {
        let kind = event
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let timestamp = event
            .get("timestamp")
            .and_then(Value::as_str)
            .map(str::to_string);
        let failed = event
            .get("status")
            .and_then(Value::as_str)
            .is_some_and(|status| {
                matches!(
                    status,
                    "failed" | "timeout" | "cancelled" | "policy_blocked" | "degraded"
                )
            })
            || kind == "transition_failed";
        match kind {
            "event_received"
            | "state_entered"
            | "state_exited"
            | "transition_started"
            | "transition_completed"
            | "transition_failed"
            | "action_started"
            | "action_completed"
            | "diagnostic_emitted" => {
                if let Some(subject) = event.get("subject").and_then(Value::as_str) {
                    mark_coverage(&mut counters, subject, failed, timestamp.clone(), None);
                }
                if matches!(
                    kind,
                    "transition_started" | "transition_completed" | "transition_failed"
                ) {
                    for field in ["from", "to"] {
                        if let Some(subject) = event.get(field).and_then(Value::as_str) {
                            mark_coverage(&mut counters, subject, false, timestamp.clone(), None);
                        }
                    }
                }
            }
            "artifact_deployed" => {
                if let Some(subject) = event.get("subject").and_then(Value::as_str) {
                    mark_coverage(
                        &mut counters,
                        subject,
                        failed,
                        timestamp.clone(),
                        Some("deployed"),
                    );
                }
            }
            _ => {}
        }
    }
    let mut subjects = counters
        .into_iter()
        .filter_map(|(subject, counter)| coverage_subject_value(subject, counter))
        .collect::<Vec<_>>();
    subjects.sort_by_key(|left| value_string(left, "subject"));
    Ok(serde_json::json!({
        "coverage_version": "0.1.0",
        "design_ir": {
            "path": design_ir.display().to_string(),
            "hash": sha256_json(ir)?
        },
        "traces": [{
            "path": trace.display().to_string(),
            "hash": sha256_json(trace_value)?
        }],
        "subjects": subjects,
        "metadata": {
            "generator": "dslraid-cli",
            "mode": "trace-derived"
        }
    }))
}

fn base_coverage_counters(ir: &dslraid_core::CoreIr) -> BTreeMap<String, CoverageCounter> {
    let mut counters = BTreeMap::new();
    for fsm in &ir.fsms {
        for state in &fsm.states {
            counters.insert(
                state_subject(&fsm.id, &state.id),
                CoverageCounter::new("state"),
            );
        }
        for event in &fsm.events {
            counters.insert(
                event_subject(&fsm.id, &event.id),
                CoverageCounter::new("event"),
            );
        }
        for guard in &fsm.guards {
            counters.insert(
                format!("guard:{}.{}", fsm.local_name(), guard.id),
                CoverageCounter::new("guard"),
            );
        }
        for action in &fsm.actions {
            counters.insert(
                format!("action:{}.{}", fsm.local_name(), action.id),
                CoverageCounter::new("action"),
            );
        }
        for transition in &fsm.transitions {
            counters.insert(
                transition_subject(&fsm.id, &transition.id),
                CoverageCounter::new("transition"),
            );
        }
    }
    for artifact in &ir.artifacts {
        counters.insert(artifact.id.clone(), CoverageCounter::new("artifact"));
    }
    counters
}

impl CoverageCounter {
    fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            count: 0,
            failures: 0,
            status_override: None,
            last_seen: None,
        }
    }
}

fn mark_coverage(
    counters: &mut BTreeMap<String, CoverageCounter>,
    subject: &str,
    failed: bool,
    timestamp: Option<String>,
    status_override: Option<&str>,
) {
    if let Some(counter) = counters.get_mut(subject) {
        counter.count += 1;
        if failed {
            counter.failures += 1;
        }
        if let Some(status_override) = status_override {
            counter.status_override = Some(status_override.to_string());
        }
        if let Some(timestamp) = timestamp {
            counter.last_seen = Some(timestamp);
        }
    }
}

fn coverage_subject_value(subject: String, counter: CoverageCounter) -> Option<Value> {
    if !matches!(
        counter.kind.as_str(),
        "state" | "transition" | "event" | "guard" | "action" | "artifact"
    ) {
        return None;
    }
    let status = if let Some(status) = counter.status_override {
        status
    } else if counter.kind == "artifact" {
        if counter.count > 0 {
            "deployed".to_string()
        } else {
            "not_deployed".to_string()
        }
    } else if counter.failures > 0 {
        "failed".to_string()
    } else if counter.count > 0 {
        "covered".to_string()
    } else {
        "uncovered".to_string()
    };
    let failure_rate = if counter.count == 0 {
        0.0
    } else {
        counter.failures as f64 / counter.count as f64
    };
    let mut value = serde_json::json!({
        "subject": subject,
        "kind": counter.kind,
        "status": status,
        "count": counter.count,
        "failure_rate": failure_rate
    });
    if let Some(last_seen) = counter.last_seen {
        value
            .as_object_mut()
            .expect("coverage subject is an object")
            .insert("last_seen".to_string(), Value::String(last_seen));
    }
    Some(value)
}

fn golden_check(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("golden path does not exist: {}", path.display());
    }
    let mut checked = 0usize;
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let _: Value = serde_json::from_slice(&fs::read(path)?)?;
            checked += 1;
        }
    }
    println!("golden ok: {checked} JSON fixture files checked");
    Ok(())
}

fn golden_update(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("golden path does not exist: {}", path.display());
    }
    println!(
        "golden update currently has no generated fixtures to refresh at {}",
        path.display()
    );
    Ok(())
}

fn project(input: &Path, projection: Option<&str>, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let view = project_view(&ir, projection, input.display().to_string())?;
    let bytes = serde_json::to_vec_pretty(&view)?;
    write_or_stdout(out, &bytes)
}

fn render(
    input: &Path,
    projection: Option<&str>,
    format: RenderFormat,
    out: Option<&Path>,
) -> Result<()> {
    let ir = load_core_ir(input)?;
    let view = project_view(&ir, projection, input.display().to_string())?;
    match out {
        Some(path) if path.extension().is_none() || path.is_dir() => {
            fs::create_dir_all(path)?;
            let stem = input
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("dslraid");
            let view_path = path.join(format!("{stem}.view.json"));
            let svg_path = path.join(format!("{stem}.svg"));
            let core_path = path.join(
                input
                    .file_name()
                    .ok_or_else(|| anyhow!("input has no file name"))?,
            );
            write_bytes(&view_path, serde_json::to_string_pretty(&view)?.as_bytes())?;
            write_bytes(&svg_path, render_svg(&view).as_bytes())?;
            fs::copy(input, core_path)?;
            println!("rendered {}", path.display());
            Ok(())
        }
        Some(path) => match format {
            RenderFormat::Svg => write_bytes(path, render_svg(&view).as_bytes()),
            RenderFormat::Json => {
                write_bytes(path, serde_json::to_string_pretty(&view)?.as_bytes())
            }
        },
        None => match format {
            RenderFormat::Svg => {
                print!("{}", render_svg(&view));
                Ok(())
            }
            RenderFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&view)?);
                Ok(())
            }
        },
    }
}

fn codegen(input: &Path, target: CliCodegenTarget, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let target: CodegenTarget = target.into();
    let generated = generate_code(&ir, target)?;
    match out {
        Some(path) if path.extension().is_none() || path.is_dir() => {
            fs::create_dir_all(path)?;
            let file = path.join(format!("dslraid_generated.{}", target.extension()));
            write_bytes(&file, generated.as_bytes())?;
            println!("generated {}", file.display());
            Ok(())
        }
        Some(path) => write_bytes(path, generated.as_bytes()),
        None => {
            print!("{generated}");
            Ok(())
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn compose(
    input: &Path,
    composition: Option<&str>,
    materialize: &str,
    limit: usize,
    focus: Option<&str>,
    depth: usize,
    format: OutputFormat,
    out: Option<&Path>,
) -> Result<()> {
    let ir = load_core_ir(input)?;
    let result = compose_result(&ir, composition, materialize, limit, focus, depth)?;
    let bytes = match format {
        OutputFormat::Json => serde_json::to_vec_pretty(&result)?,
        OutputFormat::Text => {
            let composition = result
                .get("composition")
                .ok_or_else(|| anyhow!("compose result is missing composition"))?;
            let states = result
                .get("states")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            let transitions = result
                .get("transitions")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            format!(
                "composition {} kind={} mode={} state_space={} materialized_states={} materialized_transitions={} truncated={}\n",
                composition
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or("<none>"),
                composition
                    .get("kind")
                    .and_then(Value::as_str)
                    .unwrap_or("<unknown>"),
                composition
                    .get("mode")
                    .and_then(Value::as_str)
                    .unwrap_or(materialize),
                composition
                    .get("state_space")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
                states,
                transitions,
                composition
                    .get("truncated")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            )
            .into_bytes()
        }
    };
    write_or_stdout(out, &bytes)
}

fn compose_result(
    ir: &dslraid_core::CoreIr,
    composition: Option<&str>,
    materialize: &str,
    limit: usize,
    focus: Option<&str>,
    depth: usize,
) -> Result<Value> {
    if limit == 0 {
        bail!("--limit must be greater than 0");
    }
    let selected = composition
        .and_then(|id| ir.compositions.iter().find(|item| item.id == id))
        .or_else(|| ir.compositions.first());
    match selected {
        Some(composition) => {
            let input_fsms = composition
                .inputs
                .iter()
                .map(|id| {
                    ir.find_fsm(id).ok_or_else(|| {
                        anyhow!(
                            "composition {} references unknown FSM {}",
                            composition.id,
                            id
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let state_space: usize = input_fsms
                .iter()
                .map(|fsm| fsm.states.len().max(1))
                .product();
            let mode = materialize.to_ascii_lowercase();
            let mut diagnostics = Vec::new();
            if !matches!(
                mode.as_str(),
                "diagnostics-only" | "reachable" | "reachable-only" | "focus"
            ) {
                bail!("unsupported materialization mode: {materialize}");
            }
            if state_space > limit {
                diagnostics.push(serde_json::json!({
                    "code": "CMP026",
                    "severity": "warning",
                    "message": "Composition state space exceeds materialization limit.",
                    "subjects": [composition.id]
                }));
            }
            let should_materialize = mode != "diagnostics-only";
            let (states, transitions, truncated) = if should_materialize {
                materialize_reachable_product(
                    &composition.id,
                    &input_fsms,
                    limit,
                    focus,
                    if mode == "focus" {
                        depth.max(1)
                    } else {
                        usize::MAX
                    },
                )?
            } else {
                (Vec::new(), Vec::new(), false)
            };
            Ok(serde_json::json!({
                "composition_version": "0.1.0",
                "composition": {
                    "id": composition.id,
                    "name": composition.name,
                    "kind": composition.kind,
                    "inputs": composition.inputs,
                    "mode": materialize,
                    "state_space": state_space,
                    "limit": limit,
                    "lazy": true,
                    "truncated": truncated,
                    "focus": focus,
                    "depth": depth
                },
                "states": states,
                "transitions": transitions,
                "diagnostics": diagnostics
            }))
        }
        None => Ok(serde_json::json!({
            "composition_version": "0.1.0",
            "composition": {
                "id": null,
                "name": null,
                "kind": null,
                "inputs": [],
                "mode": materialize,
                "state_space": 0,
                "limit": limit,
                "lazy": true,
                "truncated": false,
                "focus": focus,
                "depth": depth
            },
            "states": [],
            "transitions": [],
            "diagnostics": [{
                "code": "CMP000",
                "severity": "info",
                "message": "No compositions defined; nothing to compose.",
                "subjects": []
            }]
        })),
    }
}

fn materialize_reachable_product(
    composition_id: &str,
    fsms: &[&dslraid_core::Fsm],
    limit: usize,
    focus: Option<&str>,
    focus_depth: usize,
) -> Result<(Vec<Value>, Vec<Value>, bool)> {
    if fsms.is_empty() {
        return Ok((Vec::new(), Vec::new(), false));
    }
    let initial: Vec<String> = fsms
        .iter()
        .map(|fsm| {
            fsm.states
                .iter()
                .find(|state| state.initial)
                .or_else(|| fsm.states.first())
                .map(|state| state.id.clone())
                .ok_or_else(|| anyhow!("{} has no states", fsm.id))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut queue = VecDeque::from([(initial, 0usize)]);
    let mut seen = BTreeSet::new();
    let mut states = Vec::new();
    let mut transitions = Vec::new();
    let mut truncated = false;

    while let Some((tuple, depth)) = queue.pop_front() {
        let current_key = tuple_key(&tuple);
        if !seen.insert(current_key.clone()) {
            continue;
        }
        if seen.len() > limit {
            truncated = true;
            break;
        }
        if focus
            .is_none_or(|subject| tuple_matches_focus(fsms, &tuple, subject, focus_depth, depth))
        {
            states.push(tuple_state_value(composition_id, fsms, &tuple)?);
        }
        if focus.is_some() && depth >= focus_depth {
            continue;
        }
        for (index, fsm) in fsms.iter().enumerate() {
            let current = &tuple[index];
            for transition in fsm
                .transitions
                .iter()
                .filter(|transition| &transition.from == current)
            {
                let mut next_tuple = tuple.clone();
                next_tuple[index] = transition.to.clone();
                let next_key = tuple_key(&next_tuple);
                if seen.len() + queue.len() >= limit && !seen.contains(&next_key) {
                    truncated = true;
                    continue;
                }
                let edge = tuple_transition_value(
                    composition_id,
                    fsms,
                    &tuple,
                    &next_tuple,
                    &fsm.id,
                    transition,
                )?;
                if focus.is_none_or(|subject| transition_matches_focus(&edge, subject)) {
                    transitions.push(edge);
                }
                if !seen.contains(&next_key) {
                    queue.push_back((next_tuple, depth + 1));
                }
            }
        }
    }
    states.sort_by_key(|left| value_string(left, "id"));
    transitions.sort_by_key(|left| value_string(left, "id"));
    Ok((states, transitions, truncated))
}

fn tuple_state_value(
    composition_id: &str,
    fsms: &[&dslraid_core::Fsm],
    tuple: &[String],
) -> Result<Value> {
    let members = tuple_members(fsms, tuple);
    let initial = fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states
            .iter()
            .any(|state| state.id == *state_id && state.initial)
    });
    let terminal = fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states
            .iter()
            .any(|state| state.id == *state_id && state.terminal)
    });
    Ok(serde_json::json!({
        "id": tuple_subject(composition_id, &members),
        "members": members,
        "initial": initial,
        "terminal": terminal
    }))
}

fn tuple_transition_value(
    composition_id: &str,
    fsms: &[&dslraid_core::Fsm],
    from_tuple: &[String],
    to_tuple: &[String],
    fsm_id: &str,
    transition: &dslraid_core::Transition,
) -> Result<Value> {
    let from_members = tuple_members(fsms, from_tuple);
    let to_members = tuple_members(fsms, to_tuple);
    Ok(serde_json::json!({
        "id": format!("tuple_transition:{}:{}", composition_id.trim_start_matches("composition:"), transition.id),
        "from": tuple_subject(composition_id, &from_members),
        "to": tuple_subject(composition_id, &to_members),
        "members": [transition_subject(fsm_id, &transition.id)],
        "event": transition.on.as_ref().map(|event| event_subject(fsm_id, event))
    }))
}

fn tuple_members(fsms: &[&dslraid_core::Fsm], tuple: &[String]) -> Vec<String> {
    fsms.iter()
        .zip(tuple.iter())
        .map(|(fsm, state)| state_subject(&fsm.id, state))
        .collect()
}

fn tuple_subject(composition_id: &str, members: &[String]) -> String {
    format!(
        "state_tuple:{}.{}",
        composition_id.trim_start_matches("composition:"),
        members
            .iter()
            .map(|member| member.replace([':', '.'], "_"))
            .collect::<Vec<_>>()
            .join("__")
    )
}

fn tuple_key(tuple: &[String]) -> String {
    tuple.join("\u{1f}")
}

fn tuple_matches_focus(
    fsms: &[&dslraid_core::Fsm],
    tuple: &[String],
    subject: &str,
    focus_depth: usize,
    depth: usize,
) -> bool {
    depth <= focus_depth
        && tuple_members(fsms, tuple)
            .iter()
            .any(|member| member == subject)
}

fn transition_matches_focus(edge: &Value, subject: &str) -> bool {
    edge.get("members")
        .and_then(Value::as_array)
        .is_some_and(|members| {
            members
                .iter()
                .any(|member| member.as_str() == Some(subject))
        })
        || edge.get("from").and_then(Value::as_str) == Some(subject)
        || edge.get("to").and_then(Value::as_str) == Some(subject)
}

fn value_string(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

#[derive(Debug, Clone, Serialize)]
struct DiffReport {
    diff_version: &'static str,
    status: &'static str,
    base: DiffEndpoint,
    head: DiffEndpoint,
    summary: DiffSummary,
    changes: Vec<DiffChange>,
    warnings: Vec<DiffWarning>,
}

#[derive(Debug, Clone, Serialize)]
struct DiffEndpoint {
    path: String,
    hash: String,
    ir_version: String,
}

#[derive(Debug, Clone, Default, Serialize)]
struct DiffSummary {
    added: usize,
    removed: usize,
    changed: usize,
    by_kind: BTreeMap<String, DiffKindSummary>,
    review: DiffReviewSummary,
}

#[derive(Debug, Clone, Default, Serialize)]
struct DiffKindSummary {
    added: usize,
    removed: usize,
    changed: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
struct DiffReviewSummary {
    states_added: usize,
    states_removed: usize,
    states_changed: usize,
    transitions_added: usize,
    transitions_removed: usize,
    transitions_changed: usize,
    terminal_states_added: usize,
    terminal_states_removed: usize,
    terminal_paths_changed: usize,
    untested_transitions_added: usize,
    policy_traces_changed: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DiffChange {
    action: &'static str,
    kind: String,
    subject: String,
    label: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
struct DiffWarning {
    code: &'static str,
    severity: &'static str,
    subject: String,
    message: String,
}

fn diff(base: &Path, head: &Path, format: DiffFormat, out: Option<&Path>) -> Result<()> {
    let base_ir = load_core_ir(base)?;
    let head_ir = load_core_ir(head)?;
    let report = diff_report(&base_ir, &head_ir, base, head)?;
    let bytes = match format {
        DiffFormat::Json => serde_json::to_vec_pretty(&report)?,
        DiffFormat::Markdown => render_diff_markdown(&report).into_bytes(),
        DiffFormat::Text => render_diff_text(&report).into_bytes(),
    };
    write_or_stdout(out, &bytes)
}

fn diff_report(
    base_ir: &dslraid_core::CoreIr,
    head_ir: &dslraid_core::CoreIr,
    base_path: &Path,
    head_path: &Path,
) -> Result<DiffReport> {
    let base_items = query_item_map(base_ir);
    let head_items = query_item_map(head_ir);
    let base_terminal_states = terminal_state_subjects(&base_items);
    let head_terminal_states = terminal_state_subjects(&head_items);
    let mut changes = Vec::new();
    let mut warnings = Vec::new();
    let mut summary = DiffSummary::default();

    for subject in head_items
        .keys()
        .filter(|subject| !base_items.contains_key(*subject))
    {
        let after = head_items
            .get(subject)
            .expect("subject came from head item map")
            .clone();
        let change = DiffChange {
            action: "added",
            kind: item_string(&after, "kind"),
            subject: subject.clone(),
            label: item_string(&after, "label"),
            fields: Vec::new(),
            before: None,
            after: Some(after),
        };
        record_diff_change(
            &mut summary,
            &mut warnings,
            &change,
            &base_terminal_states,
            &head_terminal_states,
        );
        changes.push(change);
    }

    for subject in base_items
        .keys()
        .filter(|subject| !head_items.contains_key(*subject))
    {
        let before = base_items
            .get(subject)
            .expect("subject came from base item map")
            .clone();
        let change = DiffChange {
            action: "removed",
            kind: item_string(&before, "kind"),
            subject: subject.clone(),
            label: item_string(&before, "label"),
            fields: Vec::new(),
            before: Some(before),
            after: None,
        };
        record_diff_change(
            &mut summary,
            &mut warnings,
            &change,
            &base_terminal_states,
            &head_terminal_states,
        );
        changes.push(change);
    }

    for subject in base_items
        .keys()
        .filter(|subject| head_items.contains_key(*subject))
    {
        let before = base_items
            .get(subject)
            .expect("subject came from base item map");
        let after = head_items
            .get(subject)
            .expect("subject came from head item map");
        if before == after {
            continue;
        }
        let change = DiffChange {
            action: "changed",
            kind: item_string(after, "kind"),
            subject: subject.clone(),
            label: item_string(after, "label"),
            fields: changed_fields(before, after),
            before: Some(before.clone()),
            after: Some(after.clone()),
        };
        record_diff_change(
            &mut summary,
            &mut warnings,
            &change,
            &base_terminal_states,
            &head_terminal_states,
        );
        changes.push(change);
    }

    changes.sort_by(|left, right| {
        (left.action, left.kind.as_str(), left.subject.as_str()).cmp(&(
            right.action,
            right.kind.as_str(),
            right.subject.as_str(),
        ))
    });
    warnings.sort_by(|left, right| {
        (left.code, left.subject.as_str()).cmp(&(right.code, right.subject.as_str()))
    });
    Ok(DiffReport {
        diff_version: "0.1.0",
        status: if changes.is_empty() && warnings.is_empty() {
            "unchanged"
        } else {
            "changed"
        },
        base: DiffEndpoint {
            path: base_path.display().to_string(),
            hash: sha256_json(base_ir)?,
            ir_version: base_ir.ir_version.clone(),
        },
        head: DiffEndpoint {
            path: head_path.display().to_string(),
            hash: sha256_json(head_ir)?,
            ir_version: head_ir.ir_version.clone(),
        },
        summary,
        changes,
        warnings,
    })
}

fn query_item_map(ir: &dslraid_core::CoreIr) -> BTreeMap<String, Value> {
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

fn terminal_state_subjects(items: &BTreeMap<String, Value>) -> BTreeSet<String> {
    items
        .iter()
        .filter(|(_, item)| {
            item.get("kind").and_then(Value::as_str) == Some("state")
                && item
                    .get("terminal")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
        })
        .map(|(subject, _)| subject.clone())
        .collect()
}

fn record_diff_change(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
) {
    match change.action {
        "added" => summary.added += 1,
        "removed" => summary.removed += 1,
        "changed" => summary.changed += 1,
        _ => {}
    }
    let kind_summary = summary.by_kind.entry(change.kind.clone()).or_default();
    match change.action {
        "added" => kind_summary.added += 1,
        "removed" => kind_summary.removed += 1,
        "changed" => kind_summary.changed += 1,
        _ => {}
    }

    match (change.kind.as_str(), change.action) {
        ("state", "added") => {
            summary.review.states_added += 1;
            if change.after.as_ref().is_some_and(is_terminal_state_item) {
                summary.review.terminal_states_added += 1;
                warnings.push(diff_warning(
                    "DIF020",
                    &change.subject,
                    "terminal state added; review completion and failure paths",
                ));
            }
        }
        ("state", "removed") => {
            summary.review.states_removed += 1;
            if change.before.as_ref().is_some_and(is_terminal_state_item) {
                summary.review.terminal_states_removed += 1;
                warnings.push(diff_warning(
                    "DIF020",
                    &change.subject,
                    "terminal state removed; review completion and failure paths",
                ));
            }
        }
        ("state", "changed") => {
            summary.review.states_changed += 1;
            if change
                .fields
                .iter()
                .any(|field| field == "terminal" || field == "terminal_semantics")
            {
                warnings.push(diff_warning(
                    "DIF020",
                    &change.subject,
                    "terminal state semantics changed",
                ));
            }
        }
        ("transition", "added") => {
            summary.review.transitions_added += 1;
            if change
                .after
                .as_ref()
                .is_some_and(|item| item.get("tested").and_then(Value::as_bool) == Some(false))
            {
                summary.review.untested_transitions_added += 1;
                warnings.push(diff_warning(
                    "DIF010",
                    &change.subject,
                    "untested transition added",
                ));
            }
            if change
                .after
                .as_ref()
                .is_some_and(|item| transition_points_to_terminal(item, head_terminal_states))
            {
                summary.review.terminal_paths_changed += 1;
                warnings.push(diff_warning(
                    "DIF021",
                    &change.subject,
                    "transition adds a terminal path",
                ));
            }
            if change.after.as_ref().is_some_and(has_policy_trace) {
                summary.review.policy_traces_changed += 1;
                warnings.push(diff_warning(
                    "DIF030",
                    &change.subject,
                    "transition policy requirements added",
                ));
            }
        }
        ("transition", "removed") => {
            summary.review.transitions_removed += 1;
            if change
                .before
                .as_ref()
                .is_some_and(|item| transition_points_to_terminal(item, base_terminal_states))
            {
                summary.review.terminal_paths_changed += 1;
                warnings.push(diff_warning(
                    "DIF021",
                    &change.subject,
                    "transition removes a terminal path",
                ));
            }
            if change.before.as_ref().is_some_and(has_policy_trace) {
                summary.review.policy_traces_changed += 1;
                warnings.push(diff_warning(
                    "DIF030",
                    &change.subject,
                    "transition policy requirements removed",
                ));
            }
        }
        ("transition", "changed") => {
            summary.review.transitions_changed += 1;
            if transition_terminal_path_changed(change, base_terminal_states, head_terminal_states)
            {
                summary.review.terminal_paths_changed += 1;
                warnings.push(diff_warning(
                    "DIF021",
                    &change.subject,
                    "transition terminal path changed",
                ));
            }
            if change.fields.iter().any(|field| field == "requires") {
                summary.review.policy_traces_changed += 1;
                warnings.push(diff_warning(
                    "DIF030",
                    &change.subject,
                    "transition policy requirements changed",
                ));
            }
        }
        _ => {}
    }
}

fn diff_warning(code: &'static str, subject: &str, message: &str) -> DiffWarning {
    DiffWarning {
        code,
        severity: "warning",
        subject: subject.to_string(),
        message: message.to_string(),
    }
}

fn item_string(item: &Value, key: &str) -> String {
    item.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn changed_fields(before: &Value, after: &Value) -> Vec<String> {
    let mut keys = BTreeSet::new();
    if let Some(object) = before.as_object() {
        keys.extend(object.keys().cloned());
    }
    if let Some(object) = after.as_object() {
        keys.extend(object.keys().cloned());
    }
    keys.into_iter()
        .filter(|key| before.get(key) != after.get(key))
        .collect()
}

fn is_terminal_state_item(item: &Value) -> bool {
    item.get("kind").and_then(Value::as_str) == Some("state")
        && item
            .get("terminal")
            .and_then(Value::as_bool)
            .unwrap_or(false)
}

fn transition_points_to_terminal(item: &Value, terminal_states: &BTreeSet<String>) -> bool {
    item.get("to")
        .and_then(Value::as_str)
        .is_some_and(|state| terminal_states.contains(state))
}

fn transition_terminal_path_changed(
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
) -> bool {
    let before_terminal = change
        .before
        .as_ref()
        .is_some_and(|item| transition_points_to_terminal(item, base_terminal_states));
    let after_terminal = change
        .after
        .as_ref()
        .is_some_and(|item| transition_points_to_terminal(item, head_terminal_states));
    before_terminal != after_terminal || change.fields.iter().any(|field| field == "to")
}

fn has_policy_trace(item: &Value) -> bool {
    item.get("requires")
        .and_then(Value::as_array)
        .is_some_and(|requires| !requires.is_empty())
}

fn render_diff_text(report: &DiffReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("diff {}", report.status));
    lines.push(format!("base: {} {}", report.base.path, report.base.hash));
    lines.push(format!("head: {} {}", report.head.path, report.head.hash));
    lines.push(format!(
        "summary: +{} -{} ~{}",
        report.summary.added, report.summary.removed, report.summary.changed
    ));
    lines.push(format!(
        "fsm: states +{} -{} ~{} transitions +{} -{} ~{} terminal_paths ~{} untested_added {} policy_traces ~{}",
        report.summary.review.states_added,
        report.summary.review.states_removed,
        report.summary.review.states_changed,
        report.summary.review.transitions_added,
        report.summary.review.transitions_removed,
        report.summary.review.transitions_changed,
        report.summary.review.terminal_paths_changed,
        report.summary.review.untested_transitions_added,
        report.summary.review.policy_traces_changed
    ));
    if !report.warnings.is_empty() {
        lines.push("warnings:".to_string());
        for warning in &report.warnings {
            lines.push(format!(
                "{} {} {}: {}",
                warning.severity, warning.code, warning.subject, warning.message
            ));
        }
    }
    if !report.changes.is_empty() {
        lines.push("changes:".to_string());
        for change in &report.changes {
            let sign = match change.action {
                "added" => "+",
                "removed" => "-",
                "changed" => "~",
                _ => "?",
            };
            let fields = if change.fields.is_empty() {
                String::new()
            } else {
                format!(" fields={}", change.fields.join(","))
            };
            lines.push(format!(
                "{sign} {} {} {}{}",
                change.kind, change.subject, change.label, fields
            ));
        }
    }
    lines.join("\n")
}

fn render_diff_markdown(report: &DiffReport) -> String {
    let mut lines = Vec::new();
    lines.push("# DSLRaid Diff".to_string());
    lines.push(String::new());
    lines.push(format!("Status: **{}**", report.status));
    lines.push(format!(
        "- Base: `{}` `{}`",
        report.base.path, report.base.hash
    ));
    lines.push(format!(
        "- Head: `{}` `{}`",
        report.head.path, report.head.hash
    ));
    lines.push(String::new());
    lines.push("## Summary".to_string());
    lines.push(format!("- Added: {}", report.summary.added));
    lines.push(format!("- Removed: {}", report.summary.removed));
    lines.push(format!("- Changed: {}", report.summary.changed));
    lines.push(format!(
        "- FSM states: +{} -{} ~{}",
        report.summary.review.states_added,
        report.summary.review.states_removed,
        report.summary.review.states_changed
    ));
    lines.push(format!(
        "- FSM transitions: +{} -{} ~{}",
        report.summary.review.transitions_added,
        report.summary.review.transitions_removed,
        report.summary.review.transitions_changed
    ));
    lines.push(format!(
        "- Review flags: terminal paths ~{}, untested transitions +{}, policy traces ~{}",
        report.summary.review.terminal_paths_changed,
        report.summary.review.untested_transitions_added,
        report.summary.review.policy_traces_changed
    ));
    lines.push(String::new());
    lines.push("## Warnings".to_string());
    if report.warnings.is_empty() {
        lines.push("- none".to_string());
    } else {
        for warning in &report.warnings {
            lines.push(format!(
                "- {} `{}` `{}`: {}",
                warning.severity, warning.code, warning.subject, warning.message
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Changes".to_string());
    if report.changes.is_empty() {
        lines.push("- none".to_string());
    } else {
        lines.push("| Action | Kind | Subject | Fields |".to_string());
        lines.push("| --- | --- | --- | --- |".to_string());
        for change in &report.changes {
            let fields = if change.fields.is_empty() {
                "-".to_string()
            } else {
                change.fields.join(", ")
            };
            lines.push(format!(
                "| {} | {} | `{}` | {} |",
                change.action, change.kind, change.subject, fields
            ));
        }
    }
    lines.join("\n")
}

fn query(input: &Path, expression: &str, format: OutputFormat) -> Result<()> {
    let ir = load_core_ir(input)?;
    let items = query_values(&ir, expression)?;
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

fn query_values(ir: &dslraid_core::CoreIr, expression: &str) -> Result<Vec<Value>> {
    let filters = parse_query(expression)?;
    Ok(build_query_items(ir)
        .into_iter()
        .filter(|item| matches_query(item, &filters))
        .collect())
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

fn build_query_items(ir: &dslraid_core::CoreIr) -> Vec<Value> {
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

fn transition_endpoints(ir: &dslraid_core::CoreIr, subject: &str) -> Option<(String, String)> {
    for fsm in &ir.fsms {
        for transition in &fsm.transitions {
            if transition_subject(&fsm.id, &transition.id) == subject {
                return Some((
                    state_subject(&fsm.id, &transition.from),
                    state_subject(&fsm.id, &transition.to),
                ));
            }
        }
    }
    None
}

fn compat_check(input: &Path) -> Result<()> {
    let ir = load_core_ir(input)?;
    println!(
        "compat ok: ir_version={} project={}",
        ir.ir_version, ir.project.id
    );
    Ok(())
}

fn export(input: &Path, target: CliExportTarget, out: Option<&Path>) -> Result<()> {
    match target {
        CliExportTarget::Json => normalize(input, out),
        CliExportTarget::Svg => render(input, None, RenderFormat::Svg, out),
        CliExportTarget::Mermaid => {
            let ir = load_core_ir(input)?;
            let generated = generate_code(&ir, CodegenTarget::Mermaid)?;
            write_or_stdout(out, generated.as_bytes())
        }
        CliExportTarget::Dot => {
            let ir = load_core_ir(input)?;
            let generated = generate_code(&ir, CodegenTarget::Dot)?;
            write_or_stdout(out, generated.as_bytes())
        }
    }
}

fn print_report_text(report: &ValidationReport) {
    println!("validation {}", report.summary.status);
    println!(
        "assertions: passed={} failed={} warnings={} skipped={} n/a={}",
        report.summary.assertions.passed,
        report.summary.assertions.failed,
        report.summary.assertions.warnings,
        report.summary.assertions.skipped,
        report.summary.assertions.not_applicable
    );
    for assertion in &report.assertions {
        if assertion.status != "passed" && assertion.status != "not_applicable" {
            println!(
                "{} {} {}: {}",
                assertion.severity,
                assertion.code,
                assertion.id,
                assertion.message.clone().unwrap_or_default()
            );
            if let Some(suggestion) = &assertion.suggestion {
                println!("  suggestion: {suggestion}");
            }
        }
    }
}

fn check_json_syntax(path: impl AsRef<Path>) -> Result<()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let _: Value = serde_json::from_slice(
                &fs::read(path).with_context(|| format!("read {}", path.display()))?,
            )
            .with_context(|| format!("parse {}", path.display()))?;
        }
    }
    Ok(())
}

fn write_or_stdout(out: Option<&Path>, bytes: &[u8]) -> Result<()> {
    match out {
        Some(path) => write_bytes(path, bytes),
        None => {
            std::io::stdout().write_all(bytes)?;
            println!();
            Ok(())
        }
    }
}

fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

impl From<CliCodegenTarget> for CodegenTarget {
    fn from(value: CliCodegenTarget) -> Self {
        match value {
            CliCodegenTarget::Rust => CodegenTarget::Rust,
            CliCodegenTarget::Go => CodegenTarget::Go,
            CliCodegenTarget::Typescript => CodegenTarget::TypeScript,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_report_detects_added_untested_transition() {
        let base = load_core_ir(runscope_fixture()).unwrap();
        let mut head = base.clone();
        let fsm = head.fsms.first_mut().expect("RunScope fixture has an FSM");
        fsm.states.push(
            serde_json::from_value(serde_json::json!({
                "id": "retrying",
                "kind": "atomic"
            }))
            .unwrap(),
        );
        fsm.transitions.push(
            serde_json::from_value(serde_json::json!({
                "id": "running_to_retrying",
                "from": "running",
                "to": "retrying",
                "guards": [],
                "actions": []
            }))
            .unwrap(),
        );

        let report =
            diff_report(&base, &head, Path::new("base.json"), Path::new("head.json")).unwrap();

        assert_eq!(report.status, "changed");
        assert_eq!(report.summary.review.states_added, 1);
        assert_eq!(report.summary.review.transitions_added, 1);
        assert_eq!(report.summary.review.untested_transitions_added, 1);
        assert!(report.changes.iter().any(|change| {
            change.action == "added" && change.subject == "transition:runtime.running_to_retrying"
        }));
        assert!(report
            .warnings
            .iter()
            .any(|warning| warning.code == "DIF010"));
    }

    #[test]
    fn diff_markdown_renders_unchanged_summary() {
        let ir = load_core_ir(runscope_fixture()).unwrap();
        let report = diff_report(&ir, &ir, Path::new("base.json"), Path::new("head.json")).unwrap();
        let markdown = render_diff_markdown(&report);

        assert_eq!(report.status, "unchanged");
        assert!(markdown.contains("Status: **unchanged**"));
        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("- none"));
    }

    fn runscope_fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/runscope.raid.json")
    }
}
