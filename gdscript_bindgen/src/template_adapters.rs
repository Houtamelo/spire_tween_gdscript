use super::*;

pub fn methods(class: &Ident) -> TokenStream {
    let name = class.to_string();
    match name.as_str() {
        "Node2D" | "Node3D" => motion_methods(class),
        "Control" => control_methods(class),
        "CanvasItem" | "SpriteBase3D" => color_methods(class, "modulate"),
        "Label3D" => label_3d_methods(class),
        "Light3D" => color_methods(class, "light_color"),
        "BaseMaterial3D" => color_methods(class, "albedo_color"),
        "Label" | "RichTextLabel" => counter_methods(class),
        "Line2D" => line_2d_methods(),
        "AudioServer" => audio_server_methods(),
        "ShaderMaterial" => shader_material_methods(),
        _ => TokenStream::new(),
    }
}

fn motion_methods(class: &Ident) -> TokenStream {
    let mut out = TokenStream::new();
    let vector: Type = if class == "Node3D" { parse_quote!(Vector3) } else { parse_quote!(Vector2) };
    let property: Type =
        if class == "Node3D" { parse_quote!(SpirePropertyVec3) } else { parse_quote!(SpirePropertyVec2) };
    out.extend(quote! {
        /// Expands [param to] to a uniform absolute scale. Already registered.
        #[func]
        fn uniform_scale(node: Gd<#class>, to: real, duration: f64) -> Gd<#property> {
            gd_from_native_tween(node.do_uniform_scale(to, duration).register())
        }
    });
    for (method, doc) in [
        ("blendable_position", "Adds a local-position delta, blending with other relative tweens."),
        ("blendable_global_position", "Adds a global-position delta, blending with other relative tweens."),
        ("blendable_scale", "Adds a scale delta; this is additive, not multiplicative."),
    ] {
        let method = format_ident!("{method}");
        let native = format_ident!("do_{method}");
        out.extend(quote! {
            #[doc = #doc]
            #[func]
            fn #method(node: Gd<#class>, by: #vector, duration: f64) -> Gd<#property> {
                gd_from_native_tween(node.#native(by, duration).register())
            }
        });
    }
    for method in ["jump", "global_jump"] {
        let method = format_ident!("{method}");
        let native = format_ident!("do_{method}");
        out.extend(quote! {
            /// Jumps to a local position ([method global_jump] uses global coordinates).
            /// Captures the start on first processing. Horizontal motion is linear;
            /// vertical displacement uses OutQuad with repeated OutQuad/Yoyo hops.
            /// Positive height is upward: -Y in 2D/UI, +Y in 3D. Jump counts below
            /// one become one. Easing remaps overall progress. Supports Restart/Yoyo,
            /// not Incremental. Directly sets position; does not perform physics movement.
            #[func]
            fn #method(node: Gd<#class>, to: #vector, height: f64, jumps: i32, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(to, height, jumps, duration).register())
            }
        });
    }
    out
}

fn color_methods(class: &Ident, property: &str) -> TokenStream {
    let blend = format_ident!("blendable_{property}");
    let native_blend = format_ident!("do_{blend}");
    let gradient = format_ident!("gradient_{property}");
    let native_gradient = format_ident!("do_{gradient}");
    quote! {
        /// Captures the current color at construction, then adds [param to] minus
        /// that baseline over time. Equivalent to a relative color property tween.
        #[func]
        fn #blend(node: Gd<#class>, to: Color, duration: f64) -> Gd<SpirePropertyColor> {
            gd_from_native_tween(node.#native_blend(to, duration).register())
        }
        /// Samples the supplied Gradient with eased progress. Includes alpha by
        /// default; disabling it preserves the target's current alpha on each write.
        /// Uses the Gradient's interpolation mode/color space and samples it live.
        #[func]
        fn #gradient(node: Gd<#class>, gradient: Option<Gd<Gradient>>, duration: f64, #[opt(default = true)] include_alpha: bool) -> Option<Gd<SpireMethodFloat>> {
            let gradient = gradient.log_null_arg(|| "gradient")?;
            Some(gd_from_native_tween(node.#native_gradient(gradient, duration, include_alpha).register()))
        }
    }
}

fn counter_methods(class: &Ident) -> TokenStream {
    quote! {
        /// Replaces text with an interpolated integer. Does not parse existing text.
        /// Groups every three digits; an empty separator disables grouping. Replaces
        /// BBCode on RichTextLabel. Returns an already registered method tween.
        #[func]
        fn counter(node: Gd<#class>, from: i64, to: i64, duration: f64, #[opt(default = ",")] thousands_separator: GString) -> Gd<SpireMethodInt> {
            gd_from_native_tween(node.do_counter(from, to, duration, thousands_separator).register())
        }
    }
}

fn label_3d_methods(class: &Ident) -> TokenStream {
    let mut out = color_methods(class, "modulate");
    out.extend(counter_methods(class));
    out
}

fn control_methods(class: &Ident) -> TokenStream {
    let mut out = motion_methods(class);
    for method in ["anchor_min", "anchor_max"] {
        let method = format_ident!("{method}");
        let native = format_ident!("do_{method}");
        out.extend(quote! {
            /// Tweens the paired normalized anchors in parallel: min = left/top,
            /// max = right/bottom. Preserves the scalar anchor setters' offset behavior.
            #[func]
            fn #method(node: Gd<Control>, to: Vector2, duration: f64) -> Gd<SpireSequence> {
                gd_from_native_tween(node.#native(to, duration).register())
            }
        });
    }
    for method in ["ellipsis", "global_ellipsis"] {
        let method = format_ident!("{method}");
        let native = format_ident!("do_{method}");
        out.extend(quote! {
            /// Elliptical motion with angles in radians; positive rotation is clockwise.
            /// Uses local Control coordinates; the global variant uses global coordinates.
            /// Interpolates start/end radii. Container layout may reposition the Control.
            #[func]
            fn #method(node: Gd<Control>, center: Vector2, from_angle: f32, to_angle: f32, from_radius: Vector2, to_radius: Vector2, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(center, from_angle, to_angle, from_radius, to_radius, duration).register())
            }
        });
    }
    for method in ["circle", "global_circle"] {
        let method = format_ident!("{method}");
        let native = format_ident!("do_{method}");
        out.extend(quote! {
            /// Circular counterpart of the ellipse helper. Radius is constant; angles
            /// are radians. Uses local coordinates; global_circle uses global coordinates.
            #[func]
            fn #method(node: Gd<Control>, center: Vector2, from_angle: f32, to_angle: f32, radius: f32, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.#native(center, from_angle, to_angle, radius, duration).register())
            }
        });
    }
    out
}

fn line_2d_methods() -> TokenStream {
    quote! {
        /// Animates gradient endpoints, preserving interior stops and interpolation.
        /// Copies the gradient on first processing to preserve other users; starts
        /// from default_color when none exists. The private copy remains assigned.
        #[func]
        fn endpoint_colors(node: Gd<Line2D>, start: Color, end: Color, duration: f64) -> Gd<SpireMethodFloat> {
            gd_from_native_tween(node.do_endpoint_colors(start, end, duration).register())
        }
    }
}

fn audio_server_methods() -> TokenStream {
    let mut out = TokenStream::new();
    for method in ["bus_volume_db", "bus_volume_linear"] {
        let method = format_ident!("{method}");
        let native = format_ident!("do_{method}");
        out.extend(quote! {
            /// Tweens an existing named bus. Names are resolved each update so
            /// reordering is harmless. Removal/renaming cancels without completion.
            /// Linear gain: 0 = silence, 1 = unity, values above 1 are accepted.
            /// The db variant uses decibels. Invalid names log an error and return null.
            #[func]
            fn #method(name: StringName, to: f64, duration: f64) -> Option<Gd<SpirePropertyFloat>> {
                let tween = #native(name, to, duration).log_if_err()?;
                Some(gd_from_native_tween(tween.register()))
            }
        });
    }
    out
}

fn shader_material_methods() -> TokenStream {
    let mut out = quote! {
        /// Tweens a Color shader uniform's alpha, preserving current RGB each
        /// write. Missing/non-Color uniforms return null with an error.
        #[func]
        fn parameter_color_a(node: Gd<ShaderMaterial>, parameter: StringName, to: f64, duration: f64) -> Option<Gd<SpirePropertyFloat>> {
            let tween = do_shader_parameter_alpha(node, parameter, to, duration).log_if_err()?;
            Some(gd_from_native_tween(tween.register()))
        }
    };
    for (suffix, ty, gd) in [
        ("float", "f64", "SpirePropertyFloat"),
        ("int", "i64", "SpirePropertyInt"),
        ("vec2", "Vector2", "SpirePropertyVec2"),
        ("vec3", "Vector3", "SpirePropertyVec3"),
        ("color", "Color", "SpirePropertyColor"),
        ("vec2i", "Vector2i", "SpirePropertyVec2i"),
        ("vec3i", "Vector3i", "SpirePropertyVec3i"),
    ] {
        let method = format_ident!("parameter_{suffix}");
        let ty: Type = syn::parse_str(ty).unwrap();
        let gd = format_ident!("{gd}");
        out.extend(quote! {
            /// Tweens an existing shader uniform of the indicated type. Reads its
            /// start on first processing. Missing/incompatible uniforms return null
            /// with a diagnostic. Retains the material; shared users see the changes.
            /// Without an override, reads the renderer default. Initialize the value
            /// explicitly in headless mode, where defaults are unavailable. Replacing
            /// the Shader resource cancels existing uniform tweens.
            #[func]
            fn #method(node: Gd<ShaderMaterial>, parameter: StringName, to: #ty, duration: f64) -> Option<Gd<#gd>> {
                let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
                Some(gd_from_native_tween(tween.register()))
            }
        });
    }
    out
}
