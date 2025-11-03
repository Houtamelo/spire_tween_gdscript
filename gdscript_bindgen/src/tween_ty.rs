use super::*;

#[allow(non_camel_case_types)]
#[spire_enum::prelude::discriminant_generic_table]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenType {
    f32,
    f64,
    i32,
    i64,
    Vector2,
    Vector2i,
    Vector3,
    Vector3i,
    Color,
    String,
}

impl TweenType {
    pub fn rust_ty(self) -> Type {
        match self {
            TweenType::f32 => parse_quote! { f32 },
            TweenType::f64 => parse_quote! { f64 },
            TweenType::i32 => parse_quote! { i32 },
            TweenType::i64 => parse_quote! { i64 },
            TweenType::Vector2 => parse_quote! { Vector2 },
            TweenType::Vector3 => parse_quote! { Vector3 },
            TweenType::Color => parse_quote! { Color },
            TweenType::String => parse_quote! { GString },
            TweenType::Vector2i => parse_quote! { Vector2i },
            TweenType::Vector3i => parse_quote! { Vector3i },
        }
    }
}

#[allow(non_camel_case_types)]
#[spire_enum::prelude::discriminant_generic_table]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GdScriptType {
    int,
    float,
    Vector2,
    Vector2i,
    Vector3,
    Vector3i,
    Color,
    String,
}

impl GdScriptType {
    pub fn spire_property_suffix(self) -> Ident {
        match self {
            GdScriptType::float => format_ident!("Float"),
            GdScriptType::int => format_ident!("Int"),
            GdScriptType::Vector2 => format_ident!("Vector2"),
            GdScriptType::Vector2i => format_ident!("Vector2i"),
            GdScriptType::Vector3 => format_ident!("Vector3"),
            GdScriptType::Vector3i => format_ident!("Vector3i"),
            GdScriptType::Color => format_ident!("Color"),
            GdScriptType::String => format_ident!("String"),
        }
    }

    pub fn gdscript_ty(self) -> Ident {
        match self {
            GdScriptType::float => format_ident!("float"),
            GdScriptType::int => format_ident!("int"),
            GdScriptType::Vector2 => format_ident!("Vector2"),
            GdScriptType::Vector2i => format_ident!("Vector2i"),
            GdScriptType::Vector3 => format_ident!("Vector3"),
            GdScriptType::Color => format_ident!("Color"),
            GdScriptType::String => format_ident!("String"),
            GdScriptType::Vector3i => format_ident!("Vector3i"),
        }
    }

    pub fn rust_ty(self) -> Type {
        match self {
            GdScriptType::float => parse_quote! { f64 },
            GdScriptType::int => parse_quote! { i64 },
            GdScriptType::Vector2 => parse_quote! { Vector2 },
            GdScriptType::Vector3 => parse_quote! { Vector3 },
            GdScriptType::Color => parse_quote! { Color },
            GdScriptType::String => parse_quote! { GString },
            GdScriptType::Vector2i => parse_quote! { Vector2i },
            GdScriptType::Vector3i => parse_quote! { Vector3i },
        }
    }

    pub fn property_data_ident(self) -> Ident { format_ident!("PropertyData{}", self.spire_property_suffix()) }

    pub fn gdscript_tween_ty(self) -> Ident { format_ident!("SpireProperty{}", self.spire_property_suffix()) }
}

impl From<TweenType> for GdScriptType {
    fn from(value: TweenType) -> Self {
        match value {
            TweenType::f32 | TweenType::f64 => GdScriptType::float,
            TweenType::i32 | TweenType::i64 => GdScriptType::int,
            TweenType::Vector2 => GdScriptType::Vector2,
            TweenType::Vector2i => GdScriptType::Vector2i,
            TweenType::Vector3 => GdScriptType::Vector3,
            TweenType::Vector3i => GdScriptType::Vector3i,
            TweenType::Color => GdScriptType::Color,
            TweenType::String => GdScriptType::String,
        }
    }
}

pub fn class_property_data_ident(class: &str, ty: &GdScriptType) -> Ident {
    format_ident!("{class}{}Data", ty.spire_property_suffix())
}

pub fn class_property_data_enum_ident(class: &str, ty: &GdScriptType) -> Ident {
    format_ident!("{class}{}Kind", ty.spire_property_suffix())
}
