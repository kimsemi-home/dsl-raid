mod builder;
mod composition;
mod diagnostic;
mod edge;
mod edge_route;
mod fsm_scene;
mod ids;
mod inspector_model;
mod model;
mod node;
mod panels;
mod project;
mod scene;

pub use inspector_model::{InspectorPanel, InspectorRow, InspectorSection};
pub use model::{Layout, ViewModel, ViewSource};
pub use project::project_view;
pub use scene::{Point, StyleToken, ViewEdge, ViewNode};

pub(crate) use builder::build_fsm_view;
pub(crate) use ids::{layout_state_id, layout_transition_id};

#[cfg(test)]
mod test_fixture;
#[cfg(test)]
mod tests;
