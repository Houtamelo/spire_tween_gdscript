use super::*;

pub fn generate(classes: &[ClassData]) -> anyhow::Result<()> {
    let mut mod_names = Vec::new();

    for class in classes {
        let stream = tokenize_class_bridge(class);
        let formatted = prettyplease::unparse(&parse2(stream)?);

        let file_name = class.ident.to_string().to_case(Case::Snake);
        mod_names.push(file_name.clone());

        let path_str = format!("./spire_tween/src/gdscript_bridge/generated/{file_name}.rs");
        let path = std::path::Path::new(&path_str);
        std::fs::write(path, formatted).map_err(anyhow::Error::new)?;
    }

    let mod_stream = tokenize_root_module(&mod_names);
    let mod_formatted = prettyplease::unparse(&parse2(mod_stream)?);
    let mod_path = std::path::Path::new("./spire_tween/src/gdscript_bridge/generated/mod.rs");
    std::fs::write(mod_path, mod_formatted).map_err(anyhow::Error::new)
}

fn tokenize_root_module(mod_names: &[String]) -> TokenStream {
    let mut stream = quote! { use super::*; };

    let mod_idents = mod_names.iter().map(|name| format_ident!("{name}")).collect::<Vec<_>>();

    stream.extend(quote! {
        #(mod #mod_idents;)*
    });

    stream.extend(quote! {
        #[allow(unused_imports)]
        pub use self::{
            #( #mod_idents::* ),*
        };
    });

    stream
}

fn tokenize_class_bridge(class: &ClassData) -> TokenStream {
    let class_ident = &class.ident;

    let funcs = class.tweens_map.iter().flat_map(|(gd_ty, map)| {
        let tween_ty = gd_ty.gdscript_tween_ty();
        let rust_ty = gd_ty.rust_ty();

        map.methods.iter().map(
            move |TweenFunc {
                      property_path,
                      alias,
                      bridge_fn_name,
                      trait_fn_name,
                      doc,
                      ..
                  }| {
                let doc1 = if let Some(doc) = doc {
                    doc.clone()
                } else if let Some((left, right)) = property_path.split_once(':') {
                    format!(
                        "[b]Behavior: [/b]Tweens the `{right}` component of the property [member \
                         {class_ident}.{left}] over [param duration] seconds."
                    )
                } else {
                    format!(
                        "[b]Behavior: [/b]Tweens the property [member {class_ident}.{property_path}] over [param \
                         duration] seconds."
                    )
                };

                let doc2 =
                    format!("{doc1}\n\n[b]Returns:[/b] A handle that can be used to further customize the tween.");

                let doc2 = Lit::Str(LitStr::new(&doc2, Span::call_site()));

                let doc_attr = Attribute {
                    pound_token: Default::default(),
                    style: AttrStyle::Outer,
                    bracket_token: Default::default(),
                    meta: Meta::NameValue(MetaNameValue {
                        path: parse_quote!(doc),
                        eq_token: Default::default(),
                        value: Expr::Lit(ExprLit {
                            attrs: vec![],
                            lit:   doc2,
                        }),
                    }),
                };

                let alias = alias.iter().map(|(alias_bridge, _)| {
                    let alias_doc_str = format!("Alias for [method {}].", bridge_fn_name.unraw());
                    let alias_doc_lit = Lit::Str(LitStr::new(&alias_doc_str, Span::call_site()));

                    let alias_doc = Attribute {
                        pound_token: Default::default(),
                        style: AttrStyle::Outer,
                        bracket_token: Default::default(),
                        meta: Meta::NameValue(MetaNameValue {
                            path: parse_quote!(doc),
                            eq_token: Default::default(),
                            value: Expr::Lit(ExprLit {
                                attrs: vec![],
                                lit:   alias_doc_lit,
                            }),
                        }),
                    };
                    let non_raw = alias_bridge.unraw();
                    quote! {
                        #alias_doc
                        #[func(rename = #non_raw)]
                        fn #alias_bridge(node: Gd<#class_ident>, to: #rust_ty, duration: f64) -> Gd<#tween_ty> {
                            Self::#bridge_fn_name(node, to, duration)
                        }
                    }
                });

                let non_raw = bridge_fn_name.unraw();

                quote! {
                    #doc_attr
                    #[func(rename = #non_raw)]
                    fn #bridge_fn_name(
                        node: Gd<#class_ident>,
                        to: #rust_ty,
                        duration: f64,
                    ) -> Gd<#tween_ty> {
                        let tween = node.#trait_fn_name(to, duration).register();
                        gd_from_native_tween(tween)
                    }

                    #(#alias)*
                }
            },
        )
    });

    let class_doc = format!(
        "This class provides shortcut constructors to create tweens that animate a [{class_ident}].\n\n[b]Note:[/b] \
         This class is not meant to be instantiated. To animate properties of a base class, use the methods in the \
         \"namespace\" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name)."
    );

    let gd_bridge = &class.gdscript_bridge;
    let template_funcs = template_bridge_methods(class_ident);
    let adapter_funcs = crate::template_adapters::methods(class_ident);

    quote! {
        use super::*;

        #[doc = #class_doc]
        #[derive(GodotClass)]
        #[class(base = Object, no_init)]
        pub struct #gd_bridge {}

        #[godot_api]
        impl #gd_bridge {
            #(#funcs)*
            #template_funcs
            #adapter_funcs
        }
    }
}

/// Returns extra `#[func]` methods for template bridge functions (follow, shake, spiral, etc.)
/// that get appended to specific Do* bridge classes.
fn template_bridge_methods(class_ident: &Ident) -> TokenStream {
    let class_name = class_ident.to_string();
    let shake = shake_bridge_method(class_ident);
    match class_name.as_str() {
        "Node2D" => quote! {
            /// Moves [param node] toward [param follow_this]'s global position at [param speed] units per second,
            /// sampling the target every processing step. By default, the tween finishes once it reaches the target.
            /// For continuous following, call [code]set_loops(-1, Spire.LOOP_MODE_RESTART)[/code] on the returned handle.
            /// If [param follow_this] is freed, the tween continues toward its last known global position with the
            /// configured loop setting instead of cancelling. The returned tween is already registered.
            #[func]
            fn follow(node: Gd<Node2D>, follow_this: Gd<Node2D>, speed: f64) -> Gd<SpirePropertyVec2> {
                let tween = node.do_follow(follow_this, speed).register();
                gd_from_native_tween(tween)
            }

            #shake

            #[func]
            fn ellipsis(
                node: Gd<Node2D>,
                center: Vector2,
                from_angle: f32,
                to_angle: f32,
                from_radius: Vector2,
                to_radius: Vector2,
                duration: f64,
            ) -> Gd<SpireMethodFloat> {
                let tween = node.do_ellipsis(center, from_angle, to_angle, from_radius, to_radius, duration).register();
                gd_from_native_tween(tween)
            }

            #[func]
            fn circle(
                node: Gd<Node2D>,
                center: Vector2,
                from_angle: f32,
                to_angle: f32,
                radius: f32,
                duration: f64,
            ) -> Gd<SpireMethodFloat> {
                Self::ellipsis(node, center, from_angle, to_angle, Vector2::splat(radius), Vector2::splat(radius), duration)
            }

            #[func]
            fn spiral(
                node: Gd<Node2D>,
                center: Vector2,
                from_angle: f32,
                to_angle: f32,
                scale: Vector2,
                duration: f64,
                rotation: f32,
                shear: f32,
                mode: Spiral,
                #[opt(default = Vector2::splat(0.1))]
                log_growth: Vector2,
            ) -> Gd<SpireMethodFloat> {
                let inner = node.do_spiral(center, from_angle, to_angle, scale, duration, rotation, shear, mode, log_growth).register();
                gd_from_native_tween(inner)
            }

            #[func]
            fn contour_shape(
                node: Gd<Node2D>,
                vertices: Array<Vector2>,
                duration_or_speed: f64,
                #[opt(default = false)]
                is_speed_based: bool,
            ) -> Gd<SpireSequence> {
                let tween = node.do_contour_shape(vertices, duration_or_speed, is_speed_based).register();
                gd_from_native_tween(tween)
            }
        },
        "Node3D" => quote! {
            /// Moves [param node] toward [param follow_this]'s global position at [param speed] units per second,
            /// sampling the target every processing step. By default, the tween finishes once it reaches the target.
            /// For continuous following, call [code]set_loops(-1, Spire.LOOP_MODE_RESTART)[/code] on the returned handle.
            /// If [param follow_this] is freed, the tween continues toward its last known global position with the
            /// configured loop setting instead of cancelling. The returned tween is already registered.
            #[func]
            fn follow(node: Gd<Node3D>, follow_this: Gd<Node3D>, speed: f64) -> Gd<SpirePropertyVec3> {
                let tween = node.do_follow(follow_this, speed).register();
                gd_from_native_tween(tween)
            }

            #[func]
            fn ellipsis(
                node: Gd<Node3D>,
                center: Vector3,
                from_angle: f32,
                to_angle: f32,
                from_radius: Vector3,
                to_radius: Vector3,
                axis: Vector3,
                duration: f64,
            ) -> Gd<SpireMethodFloat> {
                let tween = node.do_ellipsis(center, from_angle, to_angle, from_radius, to_radius, axis, duration).register();
                gd_from_native_tween(tween)
            }

            /// Moves [param node] around [param center] (global coordinates) at a
            /// constant [param radius], in the plane perpendicular to [param axis].
            /// Angles are in radians; their difference controls direction and turns.
            /// The axis is normalized, with [constant Vector3.UP] used for a zero axis.
            ///
            /// Equivalent to [method ellipsis] with `(radius, radius, 0)` at both
            /// radius endpoints: there is no displacement along the axis.
            /// The returned [SpireMethodFloat] interpolates progress from 0 to 1.
            #[func]
            fn circle(
                node: Gd<Node3D>,
                center: Vector3,
                from_angle: f32,
                to_angle: f32,
                radius: f32,
                axis: Vector3,
                duration: f64,
            ) -> Gd<SpireMethodFloat> {
                let radii = Vector3::new(radius, radius, 0.0);
                Self::ellipsis(node, center, from_angle, to_angle, radii, radii, axis, duration)
            }
        },
        "Control" => quote! {
            #shake
        },
        "Skeleton3D" => quote! {
            #[func]
            fn bone_position(node: Gd<Skeleton3D>, bone_idx: i32, to: Vector3, duration: f64) -> Gd<SpirePropertyVec3> {
                let tween = node.do_bone_position(bone_idx, to, duration).register();
                gd_from_native_tween(tween)
            }

            #[func]
            fn bone_scale(node: Gd<Skeleton3D>, bone_idx: i32, to: Vector3, duration: f64) -> Gd<SpirePropertyVec3> {
                let tween = node.do_bone_scale(bone_idx, to, duration).register();
                gd_from_native_tween(tween)
            }
        },
        _ => TokenStream::new(),
    }
}

fn shake_bridge_method(class: &Ident) -> TokenStream {
    quote! {
        /// Interpolates local-position shake waypoints using DOTween's sampling math.
        /// [param strength] shapes each axis; its length is the initial waypoint magnitude.
        /// [param vibrato] times [param duration] gives the segment count, clamped to 2-65536.
        /// Nonpositive vibrato and counts above 65536 emit warnings. Short animations
        /// with positive vibrato use the minimum of two without a warning.
        /// [param randomness] is in degrees, from 0 to 180. [param fade_out] decreases
        /// magnitude and increases segment durations; otherwise durations are equal.
        /// [param blendable] replaces this tween's previous offset on the current position;
        /// disabling it writes relative to the start captured on first processing.
        /// Absolute property writers can disrupt blendable offset bookkeeping.
        /// Generates one path per playback: Restart/Incremental repeat it without drift;
        /// Yoyo retraces it. Replay regenerates; resume does not. The method value is
        /// sampled modulo one, so easing overshoot wraps. Keep its range at 0 to 1.
        /// Stop leaves position untouched. Natural/forced completion restores zero offset
        /// before finished handlers, but not necessarily before stopped handlers.
        /// Freed owners cancel. Invalid parameters emit warnings:
        /// nonpositive/nonfinite duration becomes 1 second; negative/nonfinite strength
        /// components become zero; finite randomness is clamped to 0-180 degrees,
        /// and nonfinite randomness becomes 90 degrees. Returns an already registered method tween.
        #[func]
        fn shake(
            node: Gd<#class>,
            duration: f64,
            strength: Vector2,
            #[opt(default = 10)] vibrato: i32,
            #[opt(default = 90.0)] randomness: real,
            #[opt(default = true)] fade_out: bool,
            #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
            #[opt(default = true)] blendable: bool,
        ) -> Gd<SpireMethodFloat> {
            let tween = node.do_shake(duration, strength, vibrato, randomness, fade_out, randomness_mode, blendable);
            gd_from_native_tween(tween.register())
        }
    }
}
