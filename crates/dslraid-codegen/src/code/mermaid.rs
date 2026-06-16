use anyhow::Result;
use dslraid_core::Fsm;
use std::fmt::Write;

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    writeln!(out, "stateDiagram-v2")?;
    writeln!(out, "  %% {}", fsm.name)?;
    write_initial(out, fsm)?;
    write_transitions(out, fsm)?;
    write_terminals(out, fsm)?;
    Ok(())
}

fn write_initial(out: &mut String, fsm: &Fsm) -> Result<()> {
    if let Some(initial) = fsm.states.iter().find(|state| state.initial) {
        writeln!(out, "  [*] --> {}", initial.id)?;
    }
    Ok(())
}

fn write_transitions(out: &mut String, fsm: &Fsm) -> Result<()> {
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
    Ok(())
}

fn write_terminals(out: &mut String, fsm: &Fsm) -> Result<()> {
    for state in fsm.states.iter().filter(|state| state.terminal) {
        writeln!(out, "  {} --> [*]", state.id)?;
    }
    Ok(())
}
