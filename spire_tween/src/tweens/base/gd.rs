use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct InternalUntypedTween {}

#[godot_api]
impl InternalUntypedTween {}

define_base_gd_methods! { InternalUntypedTween }
