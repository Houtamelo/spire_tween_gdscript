use super::*;

#[derive(Default)]
pub(super) struct NodeState {
    pub bound_tweens: SmolSet<[WeakAnyTween; 1]>,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeStatus {
    InsideTree,
    OutsideTreeMaybeDead,
    #[default]
    Dead,
}
