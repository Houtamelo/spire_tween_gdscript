use super::*;

macro_rules! array_bridge {
    ($name:ident, $ty:ty) => {
        #[godot_api(secondary)]
        impl Spire {
            /// Visits each destination with its corresponding duration, returning an
            /// already registered sequence. Easing applies per segment; each segment
            /// reads its start when reached. Arrays must be nonempty and equally sized;
            /// durations must be finite and nonnegative. Errors return null before
            /// registration. Nodes and Resources are supported.
            #[func]
            pub fn $name(owner: Option<Gd<Object>>, path: NodePath, values: Array<$ty>, durations: PackedFloat64Array) -> Option<Gd<SpireSequence>> {
                let owner = owner.log_null_arg(|| "owner")?;
                let sequence = do_property_array(owner, path, values.iter_shared().collect(), durations.to_vec()).log_if_err()?;
                Some(gd_from_native_tween(sequence.register()))
            }
        }
    };
}
array_bridge!(do_property_array_vec2, Vector2);
array_bridge!(do_property_array_vec3, Vector3);
