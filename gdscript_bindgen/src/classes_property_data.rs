use super::*;

pub fn generate(classes: &[ClassData]) -> anyhow::Result<()> {
    let mut mod_names = Vec::new();

    for class in classes {
        let stream = tokenize_class_properties(class);
        let formatted = prettyplease::unparse(&parse2(stream)?);
        let file_name = class.name.to_case(Case::Snake);

        let path_str = format!("./spire_tween/src/tweens/property/generated_classes_data/{file_name}.rs");
        let path = std::path::Path::new(&path_str);
        std::fs::write(path, formatted).map_err(anyhow::Error::new)?;

        mod_names.push(file_name);
    }

    let mod_stream = tokenize_root_module(&mod_names);
    let mod_formatted = prettyplease::unparse(&parse2(mod_stream)?);
    let mod_path = std::path::Path::new("./spire_tween/src/tweens/property/generated_classes_data/mod.rs");
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

fn tokenize_class_properties(class: &ClassData) -> TokenStream {
    let data_enum = tokenize_data_enum(class);
    let trait_def = tokenize_trait_definition(class);
    let trait_gd_impl = tokenize_trait_gd_impl(class);
    let trait_direct_impl = tokenize_trait_direct_impl(class);

    quote! {
        use super::*;

        #data_enum
        #trait_def
        #trait_gd_impl
        #trait_direct_impl
    }
}

#[rustfmt::skip]
fn tokenize_data_enum(class: &ClassData) -> TokenStream {
    let class_ident = &class.ident;

    let mut stream = TokenStream::new();

    #[rustfmt::skip]
    for (gd_ty, TweensMap { data_ty, enum_ty, methods}) in &class.tweens_map {
        if methods.is_empty() { continue; }
    
        let discriminants = methods.iter().map(|func| &func.enum_discrim);

        let get_cases = methods.iter().map(
            |TweenFunc { enum_discrim, get_expr, .. }| {
                quote! {
                    <#enum_ty>::#enum_discrim => {
                        let obj = &self.owner;
                        #get_expr
                    }
                }
            },
        );

        let set_cases = methods.iter().map(
            |TweenFunc { enum_discrim, set_expr, .. }| {
                quote! {
                    <#enum_ty>::#enum_discrim => {
                        let obj = &mut self.owner;
                        let val = value;
                        #set_expr
                    }
                }
            },
        );

        let get_path_cases = methods.iter().map(
            |TweenFunc { property_path,enum_discrim, .. }| {
                quote! {
                    <#enum_ty>::#enum_discrim => { NodePath::from(#property_path) }
                }
            },
        );

        let try_from_path_cases = methods.iter().map(
            | TweenFunc { property_path, enum_discrim, .. }| {
                quote! {
                    #property_path => {
                        Some(Self {
                            property: <#enum_ty>::#enum_discrim,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                }
            },
        );

        let rust_ty = gd_ty.rust_ty();

        stream.extend(quote! {
            #[derive(Debug, Clone)]
            pub struct #data_ty {
                pub property: #enum_ty,
                pub owner: Gd<#class_ident>,
                pub owner_obj_or_node: ObjectOrNode,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum #enum_ty {
                #(#discriminants,)*
            }

            impl IProperty<#rust_ty> for #data_ty {
                #[inline]
                fn get_property_value(&self) -> #rust_ty {
                    match self.property {
                        #(#get_cases)*
                    }
                }

                #[inline]
                fn set_property_value(&mut self, value: #rust_ty) {
                    match self.property {
                        #(#set_cases)*
                    }
                }
            }

            impl IPropertyData for #data_ty {
                type Target = #class_ident;

                #[inline]
                fn get_property_path(&self) -> NodePath {
                    match self.property {
                        #(#get_path_cases)*
                    }
                }

                #[inline]
                fn get_owner(&self) -> &ObjectOrNode {
                    &self.owner_obj_or_node
                }

                #[inline]
                fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
                    if let Some(casted) = match owner {
                        ObjectOrNode::Object(obj) => obj.try_cast::<#class_ident>().ok(),
                        ObjectOrNode::Node(obj) => obj.try_cast::<#class_ident>().ok(),
                    } {
                        self.owner = casted;
                        self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
                        true
                    } else {
                        false
                    }
                }
            }

            impl TryFromPathAndObject for #data_ty {
                fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
                    object.try_cast::<#class_ident>().ok().and_then(|owner| {
                        match path {
                            #(#try_from_path_cases)*
                            _ => None,
                        }
                    })
                }
            }
        })
    }

    stream
}

#[rustfmt::skip]
fn tokenize_trait_definition(class: &ClassData) -> TokenStream {
    let trait_fns_def = class.tweens_map.iter().flat_map(|(gd_ty, TweensMap { methods, .. })| {
        let rust_ty = gd_ty.rust_ty();

        methods.iter().flat_map(
            move |TweenFunc { alias, trait_fn_name, .. }| {
                let mut items = Vec::new();
                items.push(quote! {
                    fn #trait_fn_name(&self, end_val: #rust_ty, duration: f64) -> SpireTween<LerpPropertyData<#rust_ty>>;
                });

                if let Some((_, alias_trait)) = alias {
                    items.push(quote! {
                        fn #alias_trait(&self, end_val: #rust_ty, duration: f64) -> SpireTween<LerpPropertyData<#rust_ty>>;
                    });
                }

                items
            },
        )
    });

    let trait_ident = &class.trait_ident;
    quote! {
        pub trait #trait_ident<Marker = ()> {
            #(#trait_fns_def)*
        }
    }
}

#[rustfmt::skip]
fn tokenize_trait_gd_impl(class: &ClassData) -> TokenStream {
    let class_ident = &class.ident;

    let trait_fns_gd_impl = class.tweens_map.iter().flat_map(|(gd_ty, TweensMap { data_ty, enum_ty, methods })| {
        let rust_ty = gd_ty.rust_ty();

        methods.iter().flat_map(move |TweenFunc { enum_discrim, alias, trait_fn_name, .. }| {
            let mut items = Vec::new();

            items.push(quote! {
                fn #trait_fn_name(&self, end_val: #rust_ty, duration: f64) -> SpireTween<LerpPropertyData<#rust_ty >> {
                    let data = #data_ty {
                        property: <#enum_ty>::#enum_discrim,
                        owner: self.clone().upcast(),
                        owner_obj_or_node: ObjectOrNode::Node(self.clone().upcast::<#class_ident>().upcast::<Node>()),
                    };
                    SpireTween::<LerpPropertyData<#rust_ty >>::new(data.into(), Evaluator::Static(end_val), duration)
                }
            });

            if let Some((_, alias_trait)) = alias {
                items.push(
                    quote! {
                        fn #alias_trait(&self, end_val: #rust_ty, duration: f64) -> SpireTween<LerpPropertyData<#rust_ty >> {
                            let data = #data_ty {
                                property: <#enum_ty>::#enum_discrim,
                                owner: self.clone().upcast(),
                                owner_obj_or_node: ObjectOrNode::Node(self.clone().upcast::<#class_ident>().upcast::<Node>()),
                            };
                            
                            SpireTween::<LerpPropertyData<#rust_ty >>::new(data.into(), Evaluator::Static(end_val), duration)
                        }
                    });
            }

            items
        })
    });

    let trait_ident = &class.trait_ident;
    quote! {
        impl <Class: Inherits<#class_ident> + Inherits<Object>> #trait_ident<()> for Gd<Class> {
            #(#trait_fns_gd_impl)*
        }
    }
}

#[rustfmt::skip]
fn tokenize_trait_direct_impl(class: &ClassData) -> TokenStream {
    let class_ident = &class.ident;
    let trait_fns_direct_impl = class.tweens_map.iter().flat_map(
        |(gd_ty, TweensMap { data_ty, enum_ty, methods })| {
            let rust_ty = gd_ty.rust_ty();

            methods.iter().flat_map(move |TweenFunc { enum_discrim, alias, trait_fn_name, .. }| {
                let mut items = Vec::new();

                items.push(quote! {
                    fn #trait_fn_name(&self, end_val: #rust_ty, duration: f64) -> SpireTween<LerpPropertyData<#rust_ty>> {
                        let owner: Gd<#class_ident> = self.to_gd().upcast();
                        let data = #data_ty {
                            property: <#enum_ty>::#enum_discrim,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        };

                        SpireTween::<LerpPropertyData<#rust_ty>>::new(data.into(), Evaluator::Static(end_val), duration)
                    }
                });

                if let Some((_, alias_trait)) = alias {
                    items.push(quote! {
                        fn #alias_trait(&self, end_val: #rust_ty, duration: f64) -> SpireTween<LerpPropertyData<#rust_ty>> {
                            let owner: Gd<#class_ident> = self.to_gd().upcast();
                            let data = #data_ty {
                                property: <#enum_ty>::#enum_discrim,
                                owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                                owner,
                            };
                            
                            SpireTween::<LerpPropertyData<#rust_ty>>::new(data.into(), Evaluator::Static(end_val), duration)
                        }
                    });
                }

                items
            })
        });

    let trait_ident = &class.trait_ident;
    quote! {
        impl<T: WithBaseField + Inherits<#class_ident> + Inherits<Object>> #trait_ident<BaseMarker> for T {
            #(#trait_fns_direct_impl)*
        }
    }
}
