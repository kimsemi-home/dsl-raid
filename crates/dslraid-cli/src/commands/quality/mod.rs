mod agent_run;
#[cfg(test)]
mod agent_run_tests;
mod demo;
mod docs;
mod generated;
mod lisp;
mod lock;
mod projection;
mod run_cmd;
mod runtime;
mod schema;
mod semantic;
mod source_shape;

pub(crate) use run_cmd::run;
