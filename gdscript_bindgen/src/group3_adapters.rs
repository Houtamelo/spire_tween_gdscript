use super::*;

pub fn methods(class: &Ident) -> TokenStream {
    let name = class.to_string();
    match name.as_str() {
        "Node2D" => effects(class, false, false),
        "Control" => {
            let mut out = effects(class, false, false);
            out.extend(pivot());
            out
        }
        "Node3D" => {
            let mut out = effects(class, true, false);
            out.extend(orientation());
            out.extend(spiral());
            out
        }
        "Camera3D" => effects(class, true, true),
        "ScrollContainer" => scroll(),
        _ => TokenStream::new(),
    }
}

fn effects(class: &Ident, spatial: bool, camera: bool) -> TokenStream {
    let vector: Type = if spatial { parse_quote!(Vector3) } else { parse_quote!(Vector2) };
    let angular: Type = if spatial { parse_quote!(Vector3) } else { parse_quote!(f64) };
    let mut out = TokenStream::new();
    for property in ["position", "rotation", "scale"] {
        if camera {
            continue;
        }
        let method = format_ident!("punch_{property}");
        let native = format_ident!("do_{method}");
        let ty = if property == "rotation" { &angular } else { &vector };
        out.extend(quote! {
            /// Alternating decaying waypoint punch. Captures the start on first step.
            /// Rotation uses radians; scale deltas are additive. Stop leaves the
            /// target untouched; successful/forced completion restores the offset.
            /// Loops reuse the path; Yoyo retraces it. Easing remaps overall progress.
            /// Invalid duration becomes 1 second, invalid vector components become
            /// zero, elasticity clamps to 0-1, segment count clamps to 2-65536; warns.
            /// Clearing native finish connections removes restoration. Registered.
            #[func]
            fn #method(node: Gd<#class>, punch: #ty, duration: f64,
                #[opt(default = 10)] vibrato: i32, #[opt(default = 1.0)] elasticity: f64,
                #[opt(default = true)] blendable: bool) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(punch, duration, vibrato, elasticity, blendable).register())
            }
        });
    }
    for property in ["position", "rotation", "scale"] {
        if property == "position" && !spatial || camera && property == "scale" {
            continue;
        }
        let method = if property == "position" { format_ident!("shake") } else { format_ident!("shake_{property}") };
        let native = format_ident!("do_{method}");
        let ty = if property == "rotation" { &angular } else { &vector };
        out.extend(quote! {
            /// Shake using the position-shake lifecycle and input normalization.
            /// Rotation strength is in radians; scale strength is an additive delta.
            /// Camera3D position shakes in its captured relative XY plane (Z ignored).
            /// Generates once per playback, loops reuse, Yoyo retraces, replay regenerates.
            /// Stop leaves the target untouched; successful/forced completion restores.
            /// Invalid inputs warn and normalize; easing follows the global default.
            /// Clearing native finish hooks disables restoration. Already registered.
            #[func]
            fn #method(node: Gd<#class>, duration: f64, strength: #ty,
                #[opt(default = 10)] vibrato: i32, #[opt(default = 90.0)] randomness: real,
                #[opt(default = true)] fade_out: bool,
                #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
                #[opt(default = true)] blendable: bool) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(duration, strength, vibrato, randomness, fade_out, randomness_mode, blendable).register())
            }
        });
    }
    if spatial && !camera {
        out.extend(quote! {
            /// Local quaternion-composition punch with blending enabled. Radians.
            #[func]
            fn blendable_punch_rotation(node: Gd<Node3D>, punch: Vector3, duration: f64,
                #[opt(default = 10)] vibrato: i32, #[opt(default = 1.0)] elasticity: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.do_blendable_punch_rotation(punch, duration, vibrato, elasticity).register())
            }
        });
    }
    out
}

fn orientation() -> TokenStream {
    let mut out = TokenStream::new();
    for method in ["rotate", "global_rotate"] {
        let native = format_ident!("do_{method}");
        let method = format_ident!("{method}");
        out.extend(quote! {
            /// Euler target in radians, YXZ order. Fast chooses shortest angles;
            /// FastBeyond360 preserves winding; WorldAxisAdd/LocalAxisAdd compose
            /// relative rotations in world/object axes. Captures start on first step.
            /// The global variant targets world orientation. Preserves scale.
            /// Invalid values warn and normalize. Already registered float driver.
            #[func]
            fn #method(node: Gd<Node3D>, to: Vector3, duration: f64,
                #[opt(default = RotateMode::Fast)] mode: RotateMode) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(to, duration, mode).register())
            }
        });
    }
    for method in ["rotate_quaternion", "global_rotate_quaternion"] {
        let native = format_ident!("do_{method}");
        let method = format_ident!("{method}");
        out.extend(quote! {
            /// Shortest-arc quaternion interpolation. Captures start on first step;
            /// preserves scale. Target normalizes; invalid/zero targets warn and
            /// become identity. Ordinary easing applies. Returns a float method driver.
            #[func]
            fn #method(node: Gd<Node3D>, to: Quaternion, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(to, duration).register())
            }
        });
    }
    for method in ["blendable_rotation", "blendable_global_rotation"] {
        let native = format_ident!("do_{method}");
        let method = format_ident!("{method}");
        out.extend(quote! {
            /// Adds successive quaternion rotation deltas in radians. Local deltas
            /// multiply on the right; global deltas on the left. Order matters for
            /// different axes. Preserves other rotation contributions and scale.
            #[func]
            fn #method(node: Gd<Node3D>, by: Vector3, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(by, duration).register())
            }
        });
    }
    for method in ["look_at", "dynamic_look_at", "look_at_target"] {
        let native = format_ident!("do_{method}");
        let method_ident = format_ident!("{method}");
        let arg: TokenStream =
            if method == "look_at_target" { quote!(target: Gd<Node3D>) } else { quote!(towards: Vector3) };
        let value = if method == "look_at_target" { quote!(target) } else { quote!(towards) };
        out.extend(quote! {
            /// Faces a world-space point using Godot -Z forward (or +Z with
            /// use_model_front). look_at captures the destination once; dynamic_look_at
            /// recomputes it as the source moves; look_at_target also follows a moving
            /// node and binds its lifetime/pause state. Axis constraints preserve
            /// unselected initial Euler components every step via shortest Euler routes. Singular direction/up warns and
            /// retains the last valid orientation. Captures start on first step.
            #[func]
            fn #method_ident(node: Gd<Node3D>, #arg, duration: f64,
                #[opt(default = Vector3::UP)] up: Vector3,
                #[opt(default = RotationAxis::All)] axis: RotationAxis,
                #[opt(default = false)] use_model_front: bool) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(#value, duration, up, axis, use_model_front).register())
            }
        });
    }
    out
}

fn pivot() -> TokenStream {
    let mut out = TokenStream::new();
    for (method, ty) in [
        ("pivot", parse_quote!(Vector2)),
        ("pivot_x", parse_quote!(f64)),
        ("pivot_y", parse_quote!(f64)),
    ] {
        let ty: Type = ty;
        let native = format_ident!("do_{method}");
        let method = format_ident!("{method}");
        out.extend(quote! {
            /// Normalized pivot in 0-1, multiplied by the live Control size each
            /// update. Axis variants preserve the other pixel component. Captures
            /// the start ratio on first processing. Invalid targets warn and clamp.
            #[func]
            fn #method(node: Gd<Control>, to: #ty, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(to, duration).register())
            }
        });
    }
    out
}
fn scroll() -> TokenStream {
    let mut out = TokenStream::new();
    for (method, ty) in [
        ("normalized_position", parse_quote!(Vector2)),
        ("horizontal_normalized_position", parse_quote!(f64)),
        ("vertical_normalized_position", parse_quote!(f64)),
    ] {
        let ty: Type = ty;
        let native = format_ident!("do_{method}");
        let method = format_ident!("{method}");
        out.extend(quote! {
            /// Normalized scroll using live scrollbar min/max/page. X: left=0,
            /// right=1; Y: bottom=0, top=1. Empty ranges stay at minimum. Axis variants
            /// preserve the other scrollbar. Invalid targets warn and clamp to 0-1.
            #[func]
            fn #method(node: Gd<ScrollContainer>, to: #ty, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(to, duration).register())
            }
        });
    }
    out
}
fn spiral() -> TokenStream {
    quote! {
        /// Local 3D Archimedean spiral. Captures start on first step. About axis,
        /// travel=duration*speed*progress, angle=frequency*travel, radius=travel
        /// (or expands then contracts), axial displacement=depth*travel. Snapping
        /// rounds position. Invalid parameters warn and normalize. Public Pro
        /// controls are supported; undocumented numerical parity is not asserted.
        #[func]
        fn spiral(node: Gd<Node3D>, duration: f64,
            #[opt(default = Vector3::BACK)] axis: Vector3,
            #[opt(default = SpiralMode::Expand)] mode: SpiralMode,
            #[opt(default = 1.0)] speed: f64, #[opt(default = 10.0)] frequency: f64,
            #[opt(default = 0.0)] depth: f64, #[opt(default = false)] snapping: bool) -> Gd<SpireMethodFloat> {
            gd_from_native_tween(node.do_spiral(duration, axis, mode, speed, frequency, depth, snapping).register())
        }
    }
}
