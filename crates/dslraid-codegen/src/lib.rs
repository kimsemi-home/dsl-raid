use anyhow::{anyhow, Result};
use dslraid_core::{fsm_local_name, state_subject, transition_subject, CoreIr, Fsm, Projection};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

pub const VIEW_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewModel {
    pub view_version: String,
    pub source: ViewSource,
    pub layout: Layout,
    pub nodes: Vec<ViewNode>,
    pub edges: Vec<ViewEdge>,
    #[serde(default)]
    pub inspector_panels: Vec<InspectorPanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSource {
    pub core_ir: String,
    pub projection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub engine: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewNode {
    pub id: String,
    pub subject: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: String,
    #[serde(default)]
    pub badges: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewEdge {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub route: Vec<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleToken {
    pub tone: String,
    pub emphasis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorPanel {
    pub subject: String,
    pub title: String,
    pub sections: Vec<InspectorSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorSection {
    pub title: String,
    pub rows: Vec<InspectorRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorRow {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

pub fn project_view(
    ir: &CoreIr,
    projection_id: Option<&str>,
    core_path: impl Into<String>,
) -> Result<ViewModel> {
    let projection = ir
        .find_projection(projection_id)
        .ok_or_else(|| anyhow!("projection not found"))?;
    let fsm = match projection.source.as_str() {
        source if source.starts_with("fsm:") => ir
            .find_fsm(source)
            .ok_or_else(|| anyhow!("projection source {} is not an FSM", source))?,
        source => {
            return Err(anyhow!(
                "only FSM projections are implemented for MVP, got {}",
                source
            ))
        }
    };
    build_fsm_view(ir, projection, fsm, core_path.into())
}

pub fn build_fsm_view(
    ir: &CoreIr,
    projection: &Projection,
    fsm: &Fsm,
    core_path: String,
) -> Result<ViewModel> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut panels = Vec::new();
    let row_gap = 150.0;
    let col_gap = 230.0;
    let width = 168.0;
    let height = 58.0;

    for (index, state) in fsm.states.iter().enumerate() {
        let x = 80.0 + (index as f64 % 3.0) * col_gap;
        let y = 90.0 + (index as f64 / 3.0).floor() * row_gap;
        let mut badges = Vec::new();
        if state.initial {
            badges.push("initial".to_string());
        }
        if state.terminal {
            badges.push(
                state
                    .terminal_semantics
                    .clone()
                    .unwrap_or_else(|| "terminal".to_string()),
            );
        }
        badges.extend(state.tags.clone());
        let subject = state_subject(&fsm.id, &state.id);
        nodes.push(ViewNode {
            id: layout_state_id(fsm, &state.id),
            subject: subject.clone(),
            x,
            y,
            width,
            height,
            label: state.id.clone(),
            badges,
            style: Some(StyleToken {
                tone: if state.terminal { "success" } else { "default" }.to_string(),
                emphasis: if state.initial || state.terminal {
                    "strong"
                } else {
                    "normal"
                }
                .to_string(),
            }),
        });
        panels.push(state_panel(fsm, &state.id, &subject));
    }

    for transition in &fsm.transitions {
        let from = fsm
            .states
            .iter()
            .position(|state| state.id == transition.from)
            .ok_or_else(|| {
                anyhow!(
                    "transition {} has unknown from {}",
                    transition.id,
                    transition.from
                )
            })?;
        let to = fsm
            .states
            .iter()
            .position(|state| state.id == transition.to)
            .ok_or_else(|| {
                anyhow!(
                    "transition {} has unknown to {}",
                    transition.id,
                    transition.to
                )
            })?;
        let from_node = &nodes[from];
        let to_node = &nodes[to];
        let subject = transition_subject(&fsm.id, &transition.id);
        edges.push(ViewEdge {
            id: layout_transition_id(fsm, &transition.id),
            subject: subject.clone(),
            from: from_node.id.clone(),
            to: to_node.id.clone(),
            label: transition
                .on
                .clone()
                .or_else(|| Some("epsilon".to_string())),
            route: vec![
                Point {
                    x: from_node.x + from_node.width,
                    y: from_node.y + from_node.height / 2.0,
                },
                Point {
                    x: to_node.x,
                    y: to_node.y + to_node.height / 2.0,
                },
            ],
            style: Some(StyleToken {
                tone: if transition.requires.is_empty() {
                    "default"
                } else {
                    "warning"
                }
                .to_string(),
                emphasis: "normal".to_string(),
            }),
        });
        panels.push(transition_panel(fsm, transition, &subject));
    }

    panels.push(fsm_panel(ir, fsm));
    Ok(ViewModel {
        view_version: VIEW_VERSION.to_string(),
        source: ViewSource {
            core_ir: core_path,
            projection: projection.id.clone(),
            index: None,
            hash: dslraid_core::sha256_json(ir).ok(),
        },
        layout: Layout {
            engine: "manual".to_string(),
            version: VIEW_VERSION.to_string(),
        },
        nodes,
        edges,
        inspector_panels: panels,
    })
}

pub fn render_svg(view: &ViewModel) -> String {
    let max_x = view
        .nodes
        .iter()
        .map(|node| node.x + node.width)
        .fold(800.0, f64::max)
        + 80.0;
    let max_y = view
        .nodes
        .iter()
        .map(|node| node.y + node.height)
        .fold(480.0, f64::max)
        + 80.0;
    let mut svg = String::new();
    writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{max_x}" height="{max_y}" viewBox="0 0 {max_x} {max_y}" role="img" aria-label="DSLRaid projection">"#
    )
    .unwrap();
    svg.push_str(r##"<defs><marker id="arrow" markerWidth="10" markerHeight="10" refX="8" refY="3" orient="auto"><path d="M0,0 L0,6 L9,3 z" fill="#4b5563"/></marker></defs>"##);
    svg.push_str(r##"<rect width="100%" height="100%" fill="#f8fafc"/>"##);
    for edge in &view.edges {
        if edge.route.len() < 2 {
            continue;
        }
        let mut path = String::new();
        for (idx, point) in edge.route.iter().enumerate() {
            if idx == 0 {
                write!(path, "M{} {}", point.x, point.y).unwrap();
            } else {
                write!(path, " L{} {}", point.x, point.y).unwrap();
            }
        }
        writeln!(
            svg,
            r##"<path id="{}" d="{}" fill="none" stroke="#4b5563" stroke-width="2" marker-end="url(#arrow)"/>"##,
            escape(&edge.id),
            path
        )
        .unwrap();
        if let Some(label) = &edge.label {
            let a = &edge.route[0];
            let b = edge.route.last().unwrap();
            writeln!(
                svg,
                r##"<text x="{}" y="{}" fill="#334155" font-family="Inter, system-ui, sans-serif" font-size="12">{}</text>"##,
                (a.x + b.x) / 2.0,
                (a.y + b.y) / 2.0 - 8.0,
                escape(label)
            )
            .unwrap();
        }
    }
    for node in &view.nodes {
        let style = node.style.as_ref();
        let stroke = match style.map(|style| style.tone.as_str()) {
            Some("success") => "#0f766e",
            Some("warning") => "#b45309",
            Some("danger") => "#b91c1c",
            _ => "#334155",
        };
        let stroke_width = if style.map(|style| style.emphasis.as_str()) == Some("strong") {
            3
        } else {
            2
        };
        writeln!(
            svg,
            r##"<g id="{}"><rect x="{}" y="{}" width="{}" height="{}" rx="8" fill="#ffffff" stroke="{}" stroke-width="{}"/>"##,
            escape(&node.id),
            node.x,
            node.y,
            node.width,
            node.height,
            stroke,
            stroke_width
        )
        .unwrap();
        writeln!(
            svg,
            r##"<text x="{}" y="{}" fill="#0f172a" font-family="Inter, system-ui, sans-serif" font-size="15" font-weight="700">{}</text>"##,
            node.x + 14.0,
            node.y + 24.0,
            escape(&node.label)
        )
        .unwrap();
        let mut badge_x = node.x + 14.0;
        for badge in node.badges.iter().take(3) {
            let badge_width = 16.0 + badge.len() as f64 * 7.0;
            writeln!(
                svg,
                r##"<rect x="{}" y="{}" width="{}" height="18" rx="6" fill="#e2e8f0"/><text x="{}" y="{}" fill="#334155" font-family="Inter, system-ui, sans-serif" font-size="10">{}</text>"##,
                badge_x,
                node.y + 34.0,
                badge_width,
                badge_x + 8.0,
                node.y + 47.0,
                escape(badge)
            )
            .unwrap();
            badge_x += badge_width + 6.0;
        }
        svg.push_str("</g>");
    }
    svg.push_str("</svg>\n");
    svg
}

pub fn generate_code(ir: &CoreIr, target: CodegenTarget) -> Result<String> {
    let mut out = String::new();
    match target {
        CodegenTarget::Rust => {
            out.push_str("// Generated by dslraid-codegen. Do not edit by hand.\n\n");
            for fsm in &ir.fsms {
                write_rust_fsm(&mut out, fsm)?;
            }
        }
        CodegenTarget::Go => {
            out.push_str(
                "// Generated by dslraid-codegen. Do not edit by hand.\n\npackage generated\n\n",
            );
            for fsm in &ir.fsms {
                write_go_fsm(&mut out, fsm)?;
            }
        }
        CodegenTarget::TypeScript => {
            out.push_str("// Generated by dslraid-codegen. Do not edit by hand.\n\n");
            for fsm in &ir.fsms {
                write_ts_fsm(&mut out, fsm)?;
            }
        }
        CodegenTarget::Mermaid => {
            for fsm in &ir.fsms {
                write_mermaid_fsm(&mut out, fsm)?;
            }
        }
        CodegenTarget::Dot => {
            for fsm in &ir.fsms {
                write_dot_fsm(&mut out, fsm)?;
            }
        }
    }
    Ok(out)
}

#[derive(Debug, Clone, Copy)]
pub enum CodegenTarget {
    Rust,
    Go,
    TypeScript,
    Mermaid,
    Dot,
}

impl CodegenTarget {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Go => "go",
            Self::TypeScript => "ts",
            Self::Mermaid => "mmd",
            Self::Dot => "dot",
        }
    }
}

fn write_rust_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let enum_name = rust_type_name(&fsm.name);
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
    writeln!(out, "pub enum {enum_name}State {{")?;
    for state in &fsm.states {
        writeln!(out, "    {},", rust_type_name(&state.id))?;
    }
    writeln!(out, "}}\n")?;
    writeln!(out, "pub fn {}_transition(state: {enum_name}State, event: Option<&str>) -> Option<{enum_name}State> {{", snake(fsm_local_name(&fsm.id)))?;
    writeln!(out, "    match (state, event) {{")?;
    for transition in &fsm.transitions {
        let event = transition
            .on
            .as_ref()
            .map(|event| format!("Some(\"{}\")", event))
            .unwrap_or_else(|| "None".to_string());
        writeln!(
            out,
            "        ({enum_name}State::{}, {}) => Some({enum_name}State::{}),",
            rust_type_name(&transition.from),
            event,
            rust_type_name(&transition.to)
        )?;
    }
    writeln!(out, "        _ => None,")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_go_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let type_name = go_type_name(&fsm.name);
    writeln!(out, "type {type_name}State string\n")?;
    writeln!(out, "const (")?;
    for state in &fsm.states {
        writeln!(
            out,
            "\t{type_name}State{} {type_name}State = \"{}\"",
            go_type_name(&state.id),
            state.id
        )?;
    }
    writeln!(out, ")\n")?;
    writeln!(out, "func {type_name}Transition(state {type_name}State, event string) ({type_name}State, bool) {{")?;
    writeln!(out, "\tswitch state {{")?;
    for transition in &fsm.transitions {
        writeln!(
            out,
            "\tcase {type_name}State{}:",
            go_type_name(&transition.from)
        )?;
        if let Some(event) = &transition.on {
            writeln!(
                out,
                "\t\tif event == \"{}\" {{ return {type_name}State{}, true }}",
                event,
                go_type_name(&transition.to)
            )?;
        } else {
            writeln!(
                out,
                "\t\tif event == \"\" {{ return {type_name}State{}, true }}",
                go_type_name(&transition.to)
            )?;
        }
    }
    writeln!(out, "\t}}")?;
    writeln!(out, "\treturn state, false")?;
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_ts_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let const_name = rust_type_name(&fsm.name);
    writeln!(
        out,
        "export type {const_name}State = {};",
        fsm.states
            .iter()
            .map(|state| format!("\"{}\"", state.id))
            .collect::<Vec<_>>()
            .join(" | ")
    )?;
    writeln!(out, "export function {}Transition(state: {const_name}State, event?: string): {const_name}State | undefined {{", camel(fsm_local_name(&fsm.id)))?;
    writeln!(out, "  switch (`${{state}}:${{event ?? \"\"}}`) {{")?;
    for transition in &fsm.transitions {
        writeln!(
            out,
            "    case \"{}:{}\": return \"{}\";",
            transition.from,
            transition.on.as_deref().unwrap_or(""),
            transition.to
        )?;
    }
    writeln!(out, "    default: return undefined;")?;
    writeln!(out, "  }}")?;
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_mermaid_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    writeln!(out, "stateDiagram-v2")?;
    writeln!(out, "  %% {}", fsm.name)?;
    if let Some(initial) = fsm.states.iter().find(|state| state.initial) {
        writeln!(out, "  [*] --> {}", initial.id)?;
    }
    for transition in &fsm.transitions {
        if let Some(label) = &transition.on {
            writeln!(
                out,
                "  {} --> {}: {}",
                transition.from, transition.to, label
            )?;
        } else {
            writeln!(out, "  {} --> {}", transition.from, transition.to)?;
        }
    }
    for state in fsm.states.iter().filter(|state| state.terminal) {
        writeln!(out, "  {} --> [*]", state.id)?;
    }
    Ok(())
}

fn write_dot_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    writeln!(
        out,
        "digraph {} {{",
        sanitize_ident(fsm_local_name(&fsm.id))
    )?;
    writeln!(out, "  rankdir=LR;")?;
    for state in &fsm.states {
        let shape = if state.terminal {
            "doublecircle"
        } else {
            "box"
        };
        writeln!(
            out,
            "  {} [shape={} label=\"{}\"];",
            sanitize_ident(&state.id),
            shape,
            state.id
        )?;
    }
    for transition in &fsm.transitions {
        writeln!(
            out,
            "  {} -> {} [label=\"{}\"];",
            sanitize_ident(&transition.from),
            sanitize_ident(&transition.to),
            transition.on.as_deref().unwrap_or("epsilon")
        )?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

fn fsm_panel(ir: &CoreIr, fsm: &Fsm) -> InspectorPanel {
    InspectorPanel {
        subject: fsm.id.clone(),
        title: fsm.name.clone(),
        sections: vec![InspectorSection {
            title: "Summary".to_string(),
            rows: vec![
                InspectorRow {
                    label: "States".to_string(),
                    value: fsm.states.len().to_string(),
                    subject: None,
                },
                InspectorRow {
                    label: "Transitions".to_string(),
                    value: fsm.transitions.len().to_string(),
                    subject: None,
                },
                InspectorRow {
                    label: "Project".to_string(),
                    value: ir.project.name.clone(),
                    subject: Some(format!("project:{}", ir.project.id)),
                },
            ],
        }],
    }
}

fn state_panel(fsm: &Fsm, state_id: &str, subject: &str) -> InspectorPanel {
    let incoming = fsm
        .transitions
        .iter()
        .filter(|transition| transition.to == state_id)
        .count();
    let outgoing = fsm
        .transitions
        .iter()
        .filter(|transition| transition.from == state_id)
        .count();
    InspectorPanel {
        subject: subject.to_string(),
        title: state_id.to_string(),
        sections: vec![InspectorSection {
            title: "State".to_string(),
            rows: vec![
                InspectorRow {
                    label: "Parent FSM".to_string(),
                    value: fsm.id.clone(),
                    subject: Some(fsm.id.clone()),
                },
                InspectorRow {
                    label: "Incoming".to_string(),
                    value: incoming.to_string(),
                    subject: None,
                },
                InspectorRow {
                    label: "Outgoing".to_string(),
                    value: outgoing.to_string(),
                    subject: None,
                },
            ],
        }],
    }
}

fn transition_panel(
    fsm: &Fsm,
    transition: &dslraid_core::Transition,
    subject: &str,
) -> InspectorPanel {
    InspectorPanel {
        subject: subject.to_string(),
        title: transition.id.clone(),
        sections: vec![InspectorSection {
            title: "Transition".to_string(),
            rows: vec![
                InspectorRow {
                    label: "From".to_string(),
                    value: transition.from.clone(),
                    subject: Some(state_subject(&fsm.id, &transition.from)),
                },
                InspectorRow {
                    label: "To".to_string(),
                    value: transition.to.clone(),
                    subject: Some(state_subject(&fsm.id, &transition.to)),
                },
                InspectorRow {
                    label: "Event".to_string(),
                    value: transition
                        .on
                        .clone()
                        .unwrap_or_else(|| "epsilon".to_string()),
                    subject: transition
                        .on
                        .as_ref()
                        .map(|event| dslraid_core::event_subject(&fsm.id, event)),
                },
            ],
        }],
    }
}

fn layout_state_id(fsm: &Fsm, state: &str) -> String {
    format!("layout:{}.state.{}", fsm_local_name(&fsm.id), state)
}

fn layout_transition_id(fsm: &Fsm, transition: &str) -> String {
    format!(
        "layout:{}.transition.{}",
        fsm_local_name(&fsm.id),
        transition
    )
}

fn rust_type_name(input: &str) -> String {
    input
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn go_type_name(input: &str) -> String {
    rust_type_name(input)
}

fn snake(input: &str) -> String {
    input.replace(['-', '.'], "_")
}

fn camel(input: &str) -> String {
    let name = rust_type_name(input);
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_ascii_lowercase().to_string() + chars.as_str(),
        None => name,
    }
}

fn sanitize_ident(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_pascalized() {
        assert_eq!(rust_type_name("runtime-fsm"), "RuntimeFsm");
        assert_eq!(snake("runtime.fsm"), "runtime_fsm");
    }
}
