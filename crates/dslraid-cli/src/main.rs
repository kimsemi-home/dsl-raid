use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dslraid_analyzer::{validate_core_ir, ValidateOptions, ValidationReport};
use dslraid_codegen::{generate_code, project_view, render_svg, CodegenTarget};
use dslraid_core::{
    event_subject, load_core_ir, sha256_json, state_subject, transition_subject,
    validate_json_schema, CORE_SCHEMA_PATH, VALIDATION_SCHEMA_PATH, VIEW_SCHEMA_PATH,
};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    },
    Diff {
        base: PathBuf,
        head: PathBuf,
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
enum ArtifactCommand {
    Verify {
        input: PathBuf,
        #[arg(long)]
        lock: Option<PathBuf>,
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
        } => compose(&input, composition.as_deref(), &materialize),
        Command::Diff { base, head } => diff(&base, &head),
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
        Command::Artifact { command } => match command {
            ArtifactCommand::Verify { input, lock } => artifact_verify(&input, lock.as_deref()),
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
    trace_check(
        Path::new("examples/runscope/run-001.trace.json"),
        input,
        OutputFormat::Text,
    )?;
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

fn compose(input: &Path, composition: Option<&str>, materialize: &str) -> Result<()> {
    let ir = load_core_ir(input)?;
    let selected = composition
        .and_then(|id| ir.compositions.iter().find(|item| item.id == id))
        .or_else(|| ir.compositions.first());
    match selected {
        Some(composition) => {
            let state_space: usize = composition
                .inputs
                .iter()
                .filter_map(|id| ir.find_fsm(id))
                .map(|fsm| fsm.states.len())
                .product();
            println!(
                "composition {} kind={} mode={} lazy_state_space={}",
                composition.id, composition.kind, materialize, state_space
            );
        }
        None => println!("no compositions defined; nothing to compose"),
    }
    Ok(())
}

fn diff(base: &Path, head: &Path) -> Result<()> {
    let base = load_core_ir(base)?;
    let head = load_core_ir(head)?;
    let base_subjects = base.semantic_subjects();
    let head_subjects = head.semantic_subjects();
    for added in head_subjects.difference(&base_subjects) {
        println!("+ {added}");
    }
    for removed in base_subjects.difference(&head_subjects) {
        println!("- {removed}");
    }
    Ok(())
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

fn parse_query(expression: &str) -> Result<Vec<(String, String)>> {
    let expression = expression.trim();
    if expression.is_empty() || expression == "*" {
        return Ok(Vec::new());
    }
    expression
        .split(" and ")
        .map(|part| {
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| anyhow!("query clause must use key=value: {part}"))?;
            let key = key.trim();
            if key.is_empty() {
                bail!("query key cannot be empty");
            }
            let value = value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            Ok((key.to_ascii_lowercase(), value))
        })
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

fn matches_query(item: &Value, filters: &[(String, String)]) -> bool {
    filters.iter().all(|(key, expected)| {
        if key == "tag" {
            return item
                .get("tags")
                .and_then(Value::as_array)
                .is_some_and(|tags| tags.iter().any(|tag| value_matches(tag, key, expected)));
        }
        item.get(key)
            .is_some_and(|actual| value_matches(actual, key, expected))
    })
}

fn value_matches(actual: &Value, key: &str, expected: &str) -> bool {
    match actual {
        Value::Bool(value) => match expected.to_ascii_lowercase().as_str() {
            "true" | "yes" | "1" => *value,
            "false" | "no" | "0" => !*value,
            _ => false,
        },
        Value::String(value) => {
            if key == "label" || key == "path" {
                value
                    .to_ascii_lowercase()
                    .contains(&expected.to_ascii_lowercase())
            } else {
                value.eq_ignore_ascii_case(expected)
            }
        }
        Value::Number(value) => value.to_string() == expected,
        Value::Array(values) => values
            .iter()
            .any(|value| value_matches(value, key, expected)),
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

fn artifact_verify(input: &Path, _lock: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let mut errors = Vec::new();
    for artifact in &ir.artifacts {
        if artifact.kind == "generated" {
            match artifact.generated_by.as_deref() {
                Some(derivation) if ir.derivation_by_id(derivation).is_some() => {}
                Some(derivation) => errors.push(format!(
                    "{} references unknown derivation {}",
                    artifact.id, derivation
                )),
                None => errors.push(format!(
                    "{} is generated but has no generated_by",
                    artifact.id
                )),
            }
        }
    }
    if errors.is_empty() {
        println!("artifact provenance ok");
        Ok(())
    } else {
        for error in errors {
            println!("{error}");
        }
        bail!("artifact verification failed")
    }
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
