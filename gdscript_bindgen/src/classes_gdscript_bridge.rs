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
                      ..
                  }| {
                let doc1 = if let Some((left, right)) = property_path.split_once(':') {
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

                let alias = alias.as_ref().map(|(alias_bridge, _)| {
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

                    #alias
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
        }
    }
}

/// Returns extra `#[func]` methods for template bridge functions (follow, shake, spiral, etc.)
/// that get appended to specific Do* bridge classes.
fn template_bridge_methods(class_ident: &Ident) -> TokenStream {
    let class_name = class_ident.to_string();
    match class_name.as_str() {
        "Node2D" => quote! {
            #[func]
            fn follow(node: Gd<Node2D>, follow_this: Gd<Node2D>, speed: f64) -> Gd<SpirePropertyVector2> {
                let tween = node.do_follow(follow_this, speed).register();
                gd_from_native_tween(tween)
            }

            #[func]
            fn shake(
                node: Gd<Node2D>,
                radius_min: real,
                radius_max: real,
                vibratio: real,
                frequency: f64,
                duration: f64,
            ) -> Gd<SpireMethodFloat> {
                let inner = node.do_shake(radius_min, radius_max, vibratio, frequency, duration).register();
                gd_from_native_tween(inner)
            }

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
                is_speed_based: bool,
            ) -> Gd<SpireSequence> {
                let tween = node.do_contour_shape(vertices, duration_or_speed, is_speed_based).register();
                gd_from_native_tween(tween)
            }
        },
        "Node3D" => quote! {
            #[func]
            fn follow(node: Gd<Node3D>, follow_this: Gd<Node3D>, speed: f64) -> Gd<SpirePropertyVector3> {
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
                Self::ellipsis(node, center, from_angle, to_angle, Vector3::splat(radius), Vector3::splat(radius), axis, duration)
            }
        },
        "Control" => quote! {
            #[func]
            fn shake(
                node: Gd<Control>,
                radius_min: real,
                radius_max: real,
                vibratio: real,
                frequency: f64,
                duration: f64,
            ) -> Gd<SpireMethodFloat> {
                let inner = node.do_shake(radius_min, radius_max, vibratio, frequency, duration).register();
                gd_from_native_tween(inner)
            }
        },
        "Skeleton3D" => quote! {
            #[func]
            fn bone_position(node: Gd<Skeleton3D>, bone_idx: i32, to: Vector3, duration: f64) -> Gd<SpirePropertyVector3> {
                let tween = node.do_bone_position(bone_idx, to, duration).register();
                gd_from_native_tween(tween)
            }

            #[func]
            fn bone_scale(node: Gd<Skeleton3D>, bone_idx: i32, to: Vector3, duration: f64) -> Gd<SpirePropertyVector3> {
                let tween = node.do_bone_scale(bone_idx, to, duration).register();
                gd_from_native_tween(tween)
            }
        },
        _ => TokenStream::new(),
    }
}
