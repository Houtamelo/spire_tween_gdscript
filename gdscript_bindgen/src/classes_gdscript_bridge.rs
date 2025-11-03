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
                        let inner = UnsafeCell::new(node.#trait_fn_name(to, duration).register());
                        let handle = Gd::from_init_fn(|base| #tween_ty { base, inner });
                        let handle_clone = handle.clone();
                        handle.bind().to_mut().gd_handle = Some(handle_clone);
                        handle
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

    quote! {
        use super::*;

        #[doc = #class_doc]
        #[derive(GodotClass)]
        #[class(base = Object, no_init)]
        pub struct #gd_bridge {}

        #[godot_api]
        impl #gd_bridge {
            #(#funcs)*
        }
    }
}
