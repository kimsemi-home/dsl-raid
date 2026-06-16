#[derive(Debug, Clone)]
pub(crate) struct TupleNode {
    pub(crate) subject: String,
    pub(crate) members: Vec<String>,
    pub(crate) states: Vec<String>,
    pub(crate) initial: bool,
    pub(crate) terminal: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct TupleEdge {
    pub(crate) subject: String,
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) members: Vec<String>,
    pub(crate) event: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct MaterializedComposition {
    pub(crate) state_space: usize,
    pub(crate) truncated: bool,
    pub(crate) nodes: Vec<TupleNode>,
    pub(crate) edges: Vec<TupleEdge>,
}
