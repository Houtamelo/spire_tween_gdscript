use super::*;

pub(super) struct NodeState {
    pub bound_tweens: SmallSet<[WeakAnyTween; 8]>,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    InsideTree,
    OutsideTree,
    Dead {
        seen_by_process: bool,
        seen_by_physics: bool,
    },
}
