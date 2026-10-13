
// Audit-only observation: no runtime state is mutated by this class.
#[derive(GodotClass)]
#[class(base = RefCounted, no_init)]
pub struct SpireAuditDiagnostics {
    base: Base<RefCounted>,
}
#[godot_api]
impl SpireAuditDiagnostics {
    #[func]
    pub fn counts() -> GString {
        let tm = &*TM;
        let nodes = unsafe { &*tm.tracked_nodes.get() };
        let roots = unsafe { &*tm.root_tweens.get() };
        let dead = nodes.values().filter(|s| matches!(s.status, NodeStatus::Dead)).count();
        let weak: usize = nodes.values().map(|s| s.bound_tweens.len()).sum();
        GString::from(&format!("[{}, {}, {}, {}]", nodes.len(), dead, weak, roots.len()))
    }
}
