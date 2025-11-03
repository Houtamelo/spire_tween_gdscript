use super::*;

pub fn generate(classes: &[ClassData]) -> anyhow::Result<()> {
    let ty_data_enums = GdScriptTypeDiscriminantTable::filled_with(())
        .into_iter()
        .map(|(ty, _)| tokenize_ty_data_enum(ty, classes));

    let stream = quote! {
        use super::*;

        #(#ty_data_enums)*
    };

    let formatted = prettyplease::unparse(&parse2(stream)?);
    let path = std::path::Path::new("./spire_tween/src/tweens/property/generated_types_data.rs");
    std::fs::write(path, formatted).map_err(anyhow::Error::new)
}

fn tokenize_ty_data_enum(gd_ty: GdScriptType, classes: &[ClassData]) -> TokenStream {
    let enum_data = gd_ty.property_data_ident();

    let mut enum_variants_ident = Vec::new();
    let mut enum_variants_data_ty = Vec::new();

    for class in classes {
        let TweensMap { methods, .. } = &class.tweens_map[gd_ty];
        if methods.is_empty() {
            continue;
        }

        enum_variants_ident.push(class.ident.clone());
        enum_variants_data_ty.push(class_property_data_ident(&class.name, &gd_ty));
    }

    if gd_ty == GdScriptType::Vector2 {
        enum_variants_ident.push(format_ident!("Follow2D"));
        enum_variants_data_ty.push(format_ident!("PropertyVector2Node2DFollowData"));
    }

    quote! {
        #[derive(Debug, Clone)]
        #[allow(unused)]
        #[delegated_enum(impl_conversions)]
        pub enum #enum_data {
            #(#enum_variants_ident(#enum_variants_data_ty),)*
            Custom(PropertyDataCustom),
        }

        impl IGeneralPropertyData for #enum_data {
            fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
                nested_try_from_path_and_object! {
                    _path_str, owner,
                    #(#enum_variants_data_ty),*
                }

                Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
            }
        }
    }
}
