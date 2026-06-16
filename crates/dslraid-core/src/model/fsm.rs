use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Action, DefinedAt, Event, Guard, State, Transition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fsm {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub states: Vec<State>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub guards: Vec<Guard>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}
