mod composition;
mod fsm;
mod inspector_model;
mod model;
mod project;
mod scene;

pub(crate) const VIEW_VERSION: &str = "0.1.0";

pub use inspector_model::{InspectorPanel, InspectorRow, InspectorSection};
pub use model::{Layout, ViewModel, ViewSource};
pub use project::project_view;
pub use scene::{Point, StyleToken, ViewEdge, ViewNode};

#[cfg(test)]
mod test_fixture;
#[cfg(test)]
mod tests;
