use super::*;

pub(super) struct NodeState {
    pub bound_tweens: SmolSet<[WeakAnyTween; 1]>,
    pub status: NodeStatus,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            bound_tweens: SmolSet::new(),
            status: NodeStatus::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeStatus {
    InsideTree,
    OutsideTreeMaybeDead,
    #[default]
    Dead,
}
