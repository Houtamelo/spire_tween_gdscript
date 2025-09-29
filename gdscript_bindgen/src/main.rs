#![allow(unused_macros)]

use std::{
    fmt::{Formatter, Write},
    str::FromStr,
};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{IdentFragment, ToTokens, format_ident, quote};
use serde::{Deserialize, Serialize};
use syn::*;

fn main() -> anyhow::Result<()> {
    let json_str = include_str!("./../../tweenable_properties.json");
    let classes_json: Vec<ClassJson> = serde_json::from_str(json_str)?;
    let mut classes = classes_json
        .into_iter()
        .map(gen_class_data)
        .collect::<Vec<_>>();

    classes.sort_by_key(|class| class.name.clone());

    create_rust_bridge_files(&classes)?;
    create_gd_tween_builder_file(&classes)?;
    create_gd_handles_files()?;

    Ok(())
}

fn create_gd_handles_files() -> anyhow::Result<()> {
    let tys = [
        ("float", "F32", "0.0"),
        ("float", "F64", "0.0"),
        ("int", "I32", "0"),
        ("int", "I64", "0"),
        ("Vector2", "Vector2", "Vector2.ZERO"),
        ("Vector3", "Vector3", "Vector3.ZERO"),
        ("Color", "Color", "Color(0, 0, 0, 0)"),
        ("String", "String", "\"\""),
    ];

    for (ty, ty_name, default) in &tys {
        create_gd_handle(ty, ty_name, default)?;
    }

    Ok(())
}

fn create_gd_handle(ty: &str, ty_name: &str, default: &str) -> anyhow::Result<()> {
    let contents = format!(
        "\
class_name TweenProperty{ty_name}
const State = preload(\"res://handle/tween_handle.gd\").State

var tween: InternalTweenProperty{ty_name}

func _init(p_tween: InternalTweenProperty{ty_name}):
    tween = p_tween

var state: State:
    get: return tween.get_state() as State
    set(val): tween.set_state(val)

var is_playing: bool:
    get: return tween.is_playing()
    set(val):
        if val: tween.play()
        else: tween.pause()

var is_paused: bool:
    get: return tween.is_paused()
    set(val):
        if val: tween.pause()
        else: tween.play()

func play() -> void: tween.play()

func pause() -> void: tween.pause()

func stop() -> void: tween.stop()

var property_path: NodePath:
    get: return tween.get_property_path()
    set(val): tween.set_property_path(val)

var owner: Object:
    get: return tween.get_owner()
    set(val): tween.set_owner(val)

var easing: Variant:
    get: return tween.get_ease()
    set(val): tween.set_ease(val)

var duration: float:
    get: return tween.get_duration()
    set(val): tween.set_duration(val)

var start_value: {ty}:
    set(val): tween.set_start_value(val)

var final_value: {ty}:
    get: return tween.get_final_value()
    set(val): tween.set_final_value(val)

func is_absolute() -> bool: return tween.is_absolute()

func is_relative() -> bool: return tween.is_relative()

func is_speed_based() -> bool: return tween.is_speed_based()

## Builder methods

func from(value: {ty}) -> TweenProperty{ty_name}:
    tween.set_start_value(value)
    return self

func with_ease(p_ease: Variant) -> TweenProperty{ty_name}:
    tween = tween.with_ease(p_ease)
    return self

func set_absolute() -> TweenProperty{ty_name}:
    tween.set_absolute()
    return self

func set_speed_based(speed: float) -> TweenProperty{ty_name}:
    tween.set_speed_based(speed)
    return self

func set_relative(to: {ty} = {default}) -> TweenProperty{ty_name}:
    tween.set_relative(to)
    return self
"
    );

    let path_str = format!(
        "./spire_tween_gdscript/handle/property/tween_property_{}.gd",
        ty_name.to_lowercase()
    );
    let path = std::path::Path::new(&path_str);
    std::fs::write(path, contents).map_err(anyhow::Error::new)
}

fn create_rust_bridge_files(classes: &[ClassData]) -> anyhow::Result<()> {
    let mut modules = Vec::new();

    for class in classes {
        let stream = gen_rust_bridge(class);
        let formatted = prettyplease::unparse(&parse2(stream)?);

        let file_name = class.name.to_case(Case::Snake);
        modules.push(file_name.clone());

        let path_str =
            format!("./spire_tween/src/tweens/property/named_datas/generated/{file_name}.rs");
        let path = std::path::Path::new(&path_str);
        std::fs::write(path, formatted).map_err(anyhow::Error::new)?;
    }

    let mut mod_file = String::with_capacity(256 * 16);
    writeln!(mod_file, "\nuse super::*;\n")?;

    for module in &modules {
        writeln!(mod_file, "pub use {module}::*;")?;
        writeln!(mod_file, "#[allow(unused_imports)]\nmod {module};")?;
    }

    let all_tys = TweenTypeDiscriminantTable::filled_with(())
        .iter()
        .map(|(ty, _)| ty)
        .collect::<Vec<_>>();

    for ty in all_tys {
        let enum_data = format_ident!("Property{}Data", ty);

        let mut enum_variants_ident = Vec::new();
        let mut enum_variants_data_ty = Vec::new();

        for class in classes {
            let TweensMap { methods, .. } = &class.tweens_map[ty];
            if methods.is_empty() {
                continue;
            }

            enum_variants_ident.push(class.ident.clone());
            enum_variants_data_ty.push(data_ty_ident(&class.name, &ty));
        }

        if ty == TweenType::Vector2 {
            enum_variants_ident.push(format_ident!("Follow2D"));
            enum_variants_data_ty.push(format_ident!("PropertyVector2Node2DFollowData"));
        }

        let tokens = quote! {
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
        };

        let formatted = prettyplease::unparse(&parse2(tokens)?);
        writeln!(mod_file, "{formatted}\n")?;
    }

    let mod_path =
        std::path::Path::new("./spire_tween/src/tweens/property/named_datas/generated/mod.rs");
    std::fs::write(mod_path, mod_file).map_err(anyhow::Error::new)
}

fn create_gd_tween_builder_file(classes: &[ClassData]) -> anyhow::Result<()> {
    let mut tweens_by_group = ClassGroupDiscriminantTable::filled_with(Vec::new());

    {
        let mut builder = String::with_capacity(256 * 256);
        write!(builder, "class_name Spire\n\n")?;

        for ClassData {
            ident: class_ident,
            gd_bridge,
            tweens_map,
            group,
            ..
        } in classes
        {
            if let Some(group) = group {
                tweens_by_group[*group].push((class_ident, gd_bridge, tweens_map));
                continue;
            }

            write_gd_class_tweens(&mut builder, class_ident, gd_bridge, tweens_map)?;
        }

        let path = std::path::Path::new("./spire_tween_gdscript/spire.gd");
        std::fs::write(path, builder).map_err(anyhow::Error::new)?;
    }

    // Grouped classes
    for (group, tweens) in tweens_by_group {
        let mut builder = String::with_capacity(256 * 256);

        let (group_class_name, file_name) = match group {
            ClassGroup::Control => ("SpireUI", "spire_ui"),
            ClassGroup::Node2D => ("Spire2D", "spire_2d"),
            ClassGroup::Node3D => ("Spire3D", "spire_3d"),
            ClassGroup::Misc => ("SpireMisc", "spire_misc"),
        };

        write!(builder, "class_name {group_class_name}\n\n")?;

        match group {
            ClassGroup::Control => {}
            ClassGroup::Node2D => {
                // Custom methods
                {
                    write!(
                        builder,
                        "static func do_follow(node: Node2D, follow_this: Node2D, speed: float) -> TweenPropertyVector2:\n\
                        \treturn TweenPropertyVector2.new(CustomTweener.node2d_do_follow(node, follow_this, speed))\n\n"
                    )?;
                }
            }
            ClassGroup::Node3D => {}
            ClassGroup::Misc => {}
        }

        for (class_ident, gd_bridge, tweens_map) in tweens {
            write_gd_class_tweens(&mut builder, class_ident, gd_bridge, tweens_map)?;
        }

        let path_str = format!("./spire_tween_gdscript/{file_name}.gd");
        let path = std::path::Path::new(&path_str);
        std::fs::write(path, builder).map_err(anyhow::Error::new)?;
    }

    Ok(())
}

fn write_gd_class_tweens(
    builder: &mut String,
    class_ident: &Ident,
    gd_bridge: &Ident,
    tweens_map: &TweenTypeDiscriminantTable<TweensMap>,
) -> std::fmt::Result {
    write!(builder, "## {class_ident}\n\n")?;

    for (ty, TweensMap { methods, .. }) in tweens_map {
        let gd_ty = gd_ty(ty);
        let gd_tween_ty = gd_tween_ty(ty);

        for TweenFunc { func, alias, .. } in methods {
            write!(
                builder,
                "static func {func}(node: {class_ident}, to: {gd_ty}, duration: float) -> {gd_tween_ty}:\n\
                    \treturn {gd_tween_ty}.new({gd_bridge}.{func}(node, to, duration))\n\n"
            )?;

            if let Some(alias) = alias {
                write!(
                    builder,
                    "static func {alias}(node: {class_ident}, to: {gd_ty}, duration: float) -> {gd_tween_ty}:\n\
                    \treturn {gd_tween_ty}.new({gd_bridge}.{func}(node, to, duration))\n\n"
                )?;
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ClassJson {
    class:  String,
    tweens: Vec<TweenJson>,
    group:  Option<ClassGroup>,
}

struct ClassData {
    name: String,
    ident: Ident,
    trait_ident: Ident,
    gd_bridge: Ident,
    group: Option<ClassGroup>,
    tweens_map: TweenTypeDiscriminantTable<TweensMap>,
}

#[derive(Serialize, Deserialize)]
struct TweenJson {
    name: String,
    ty: TweenType,
    func: Option<String>,
    alias: Option<String>,
    get: Option<String>,
    set: Option<String>,
}

#[spire_enum::prelude::discriminant_generic_table]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ClassGroup {
    #[serde(rename = "UI")]
    Control,
    #[serde(rename = "2D")]
    Node2D,
    #[serde(rename = "3D")]
    Node3D,
    Misc,
}

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

impl IdentFragment for TweenType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TweenType::f32 => write!(f, "F32"),
            TweenType::f64 => write!(f, "F64"),
            TweenType::i32 => write!(f, "I32"),
            TweenType::i64 => write!(f, "I64"),
            TweenType::Vector2 => write!(f, "Vector2"),
            TweenType::Vector2i => write!(f, "Vector2I"),
            TweenType::Vector3 => write!(f, "Vector3"),
            TweenType::Color => write!(f, "Color"),
            TweenType::String => write!(f, "GString"),
            TweenType::Vector3i => write!(f, "Vector3I"),
        }
    }
}

impl ToTokens for TweenType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TweenType::f32 => tokens.extend(quote! { f32 }),
            TweenType::f64 => tokens.extend(quote! { f64 }),
            TweenType::i32 => tokens.extend(quote! { i32 }),
            TweenType::i64 => tokens.extend(quote! { i64 }),
            TweenType::Vector2 => tokens.extend(quote! { Vector2 }),
            TweenType::Vector3 => tokens.extend(quote! { Vector3 }),
            TweenType::Color => tokens.extend(quote! { Color }),
            TweenType::String => tokens.extend(quote! { GString }),
            TweenType::Vector2i => tokens.extend(quote! { Vector2i }),
            TweenType::Vector3i => tokens.extend(quote! { Vector3i }),
        }
    }
}

struct TweensMap {
    data_ty: Ident,
    enum_ty: Ident,
    methods: Vec<TweenFunc>,
}

#[derive(Clone)]
struct TweenFunc {
    func: Ident,
    alias: Option<Ident>,
    property_path: String,
    enum_discrim: Ident,
    get_expr: TokenStream,
    set_expr: TokenStream,
}

impl TweenFunc {
    fn direct(
        TweenJson {
            name,
            func,
            alias,
            ty,
            get,
            set,
        }: &TweenJson,
    ) -> Self {
        let property_path = name.clone();

        let func = func
            .as_ref()
            .map(|s| format_ident!("{s}"))
            .unwrap_or_else(|| property_to_gd_method(&property_path));

        let enum_discrim = format_ident!("{}", name.to_case(Case::Pascal));

        let get_expr = get
            .as_ref()
            .map(|str| TokenStream::from_str(str).unwrap())
            .unwrap_or_else(|| {
                let get_ident = format_ident!("get_{}", name);
                quote! { obj.#get_ident() }
            });

        let set_expr = set
            .as_ref()
            .map(|str| TokenStream::from_str(str).unwrap())
            .unwrap_or_else(|| {
                let set_ident = format_ident!("set_{}", name);

                if let TweenType::String = ty {
                    quote! { obj.#set_ident(&val); }
                } else {
                    quote! { obj.#set_ident(val); }
                }
            });

        let alias = alias.as_ref().map(|s| format_ident!("{s}"));

        Self {
            property_path,
            func,
            enum_discrim,
            get_expr,
            set_expr,
            alias,
        }
    }

    fn indexed(
        inner_field: &str,
        TweenJson {
            name,
            func,
            alias,
            get,
            set,
            ..
        }: &TweenJson,
    ) -> Self {
        let property_path = format!("{name}:{inner_field}");

        let func = func
            .as_ref()
            .map(|s| format_ident!("{s}_{inner_field}"))
            .unwrap_or_else(|| property_to_gd_method(&property_path));

        let inner_field_ident = format_ident!("{inner_field}");
        let enum_discrim =
            format_ident!("{}{}", name.to_case(Case::Pascal), inner_field.to_uppercase());

        let get_ident = format_ident!("get_{}", name);

        let get_expr = if let Some(get) = get {
            let get_ts = TokenStream::from_str(get).unwrap();
            quote! { #get_ts.#inner_field_ident }
        } else {
            quote! { obj.#get_ident().#inner_field_ident }
        };

        let set_expr = if let Some(set) = set {
            let set_ts = TokenStream::from_str(set).unwrap();

            quote! {
                let mut prop_val = #get_expr;
                prop_val.#inner_field_ident = val;
                let val = prop_val;
                #set_ts
            }
        } else {
            let set_ident = format_ident!("set_{}", name);

            quote! {
                let mut prop_val = obj.#get_ident();
                prop_val.#inner_field_ident = val;
                obj.#set_ident(prop_val);
            }
        };

        let alias = alias.as_ref().map(|s| format_ident!("{s}_{inner_field}"));

        Self {
            property_path,
            func,
            enum_discrim,
            get_expr,
            set_expr,
            alias,
        }
    }
}

fn gen_class_data(class_json: ClassJson) -> ClassData {
    let ClassJson { class, group, .. } = &class_json;
    let class_ident = format_ident!("{class}");
    let trait_ident = format_ident!("Do{class_ident}");
    let gd_bridge = gd_bridge_ty(&class);
    let tweens_map = gen_class_type_tweens_map(&class_json);

    ClassData {
        name: class.clone(),
        ident: class_ident,
        trait_ident,
        gd_bridge,
        tweens_map,
        group: *group,
    }
}

fn gen_class_type_tweens_map(
    ClassJson { class, tweens, .. }: &ClassJson,
) -> TweenTypeDiscriminantTable<TweensMap> {
    let mut type_tweens_map = TweenTypeDiscriminantTable::from_fn(|ty| {
        TweensMap {
            data_ty: data_ty_ident(&class, &ty),
            enum_ty: enum_ty_ident(&class, &ty),
            methods: Vec::new(),
        }
    });

    for tween_json in tweens {
        type_tweens_map[tween_json.ty]
            .methods
            .push(TweenFunc::direct(&tween_json));

        match tween_json.ty {
            TweenType::Vector2 => {
                type_tweens_map[TweenType::f32].methods.extend([
                    TweenFunc::indexed("x", tween_json),
                    TweenFunc::indexed("y", tween_json),
                ]);
            }
            TweenType::Vector2i => {
                type_tweens_map[TweenType::i32].methods.extend([
                    TweenFunc::indexed("x", tween_json),
                    TweenFunc::indexed("y", tween_json),
                ]);
            }
            TweenType::Vector3 => {
                type_tweens_map[TweenType::f32].methods.extend([
                    TweenFunc::indexed("x", tween_json),
                    TweenFunc::indexed("y", tween_json),
                    TweenFunc::indexed("z", tween_json),
                ]);
            }
            TweenType::Vector3i => {
                type_tweens_map[TweenType::i32].methods.extend([
                    TweenFunc::indexed("x", tween_json),
                    TweenFunc::indexed("y", tween_json),
                    TweenFunc::indexed("z", tween_json),
                ]);
            }

            TweenType::Color => {
                type_tweens_map[TweenType::f32].methods.extend([
                    TweenFunc::indexed("r", tween_json),
                    TweenFunc::indexed("g", tween_json),
                    TweenFunc::indexed("b", tween_json),
                    TweenFunc::indexed("a", tween_json),
                ]);
            }
            | TweenType::f32
            | TweenType::f64
            | TweenType::i32
            | TweenType::i64
            | TweenType::String => {}
        }
    }

    type_tweens_map
}

fn gen_rust_bridge(class: &ClassData) -> TokenStream {
    let ClassData {
        ident: class_ident,
        trait_ident,
        gd_bridge,
        tweens_map,
        ..
    } = class;

    let ty_datas_def = tweens_map.iter().filter_map(
        |(
            ty,
            TweensMap {
                data_ty,
                enum_ty,
                methods,
            },
        )| {
            if methods.is_empty() {
                return None;
            }

            let discriminants = methods
                .iter()
                .map(|TweenFunc { enum_discrim, .. }| enum_discrim);

            let get_cases = methods.iter().map(
                |TweenFunc {
                     enum_discrim,
                     get_expr,
                     ..
                 }| {
                    quote! {
                        <#enum_ty>::#enum_discrim => {
                            let obj = &self.owner;
                            #get_expr
                        }
                    }
                },
            );

            let set_cases = methods.iter().map(
                |TweenFunc {
                     enum_discrim,
                     set_expr,
                     ..
                 }| {
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
                |TweenFunc {
                     property_path,
                     enum_discrim,
                     ..
                 }| {
                    quote! {
                        <#enum_ty>::#enum_discrim => { NodePath::from(#property_path) }
                    }
                },
            );

            let try_from_path_cases = methods.iter().map(
                |TweenFunc {
                     property_path,
                     enum_discrim,
                     ..
                 }| {
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

            Some(quote! {
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

                impl IProperty<#ty> for #data_ty {
                    #[inline]
                    fn get_property_value(&self) -> #ty {
                        match self.property {
                            #(#get_cases)*
                        }
                    }

                    #[inline]
                    fn set_property_value(&mut self, value: #ty) {
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
        },
    );

    let trait_fns_def = tweens_map.iter().flat_map(|(ty, TweensMap { methods, .. })| {
        methods.iter().flat_map(
            move |TweenFunc {
                func, alias, ..
            }| {
                let mut items = Vec::new();
                items.push(
                    quote! {
                        fn #func(&self, end_val: #ty, duration: f64) -> SpireTween<LerpPropertyData<#ty>>;
                    });

                if let Some(alias) = alias {
                    items.push(
                        quote! {
                            fn #alias(&self, end_val: #ty, duration: f64) -> SpireTween<LerpPropertyData<#ty>>;
                        });
                }

                items
            },
        )
    });

    let trait_def = quote! {
        pub trait #trait_ident<Marker = ()> {
            #(#trait_fns_def)*
        }
    };

    let trait_fns_gd_impl = tweens_map.iter().flat_map(|(ty, TweensMap { data_ty, enum_ty, methods })| {
        methods.iter().flat_map(move |TweenFunc { func, enum_discrim, alias, .. }| {
            let mut items = Vec::new();

            items.push(
                quote! {
                fn #func(&self, end_val: #ty, duration: f64) -> SpireTween<LerpPropertyData<#ty>> {
                    let data = #data_ty {
                        property: <#enum_ty>::#enum_discrim,
                        owner: self.clone().upcast(),
                        owner_obj_or_node: ObjectOrNode::Node(self.clone().upcast::<#class_ident>().upcast::<Node>()),
                    };
                    SpireTween::<LerpPropertyData<#ty>>::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
                }
            });

            if let Some(alias) = alias {
                items.push(
                    quote! {
                        fn #alias(&self, end_val: #ty, duration: f64) -> SpireTween<LerpPropertyData<#ty>> {
                            let data = #data_ty {
                                property: <#enum_ty>::#enum_discrim,
                                owner: self.clone().upcast(),
                                owner_obj_or_node: ObjectOrNode::Node(self.clone().upcast::<#class_ident>().upcast::<Node>()),
                            };
                            SpireTween::<LerpPropertyData<#ty>>::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
                        }
                    });
            }

            items
        })
    });

    let trait_gd_impl = quote! {
        impl <Class: Inherits<#class_ident> + Inherits<Object>> #trait_ident<()> for Gd<Class> {
            #(#trait_fns_gd_impl)*
        }
    };

    let trait_fns_direct_impl = tweens_map.iter().flat_map(|(ty, TweensMap { data_ty, enum_ty, methods })| {
        let class_ident = class_ident.clone();

        methods.iter().flat_map(move |TweenFunc { func, enum_discrim, alias, .. }| {
            let mut items = Vec::new();
            items.push(quote! {
                fn #func(&self, end_val: #ty, duration: f64) -> SpireTween<LerpPropertyData<#ty>> {
                    let owner: Gd<#class_ident> = self.to_gd().upcast();
                    let data = #data_ty {
                        property: <#enum_ty>::#enum_discrim,
                        owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                        owner,
                    };
                    SpireTween::<LerpPropertyData<#ty>>::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
                }
            });

            if let Some(alias) = alias {
                items.push(quote! {
                    fn #alias(&self, end_val: #ty, duration: f64) -> SpireTween<LerpPropertyData<#ty>> {
                        let owner: Gd<#class_ident> = self.to_gd().upcast();
                        let data = #data_ty {
                            property: <#enum_ty>::#enum_discrim,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        };
                        SpireTween::<LerpPropertyData<#ty>>::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
                    }
                });
            }

            items
        })
    });

    let trait_direct_impl = quote! {
        impl<T: WithBaseField + Inherits<#class_ident> + Inherits<Object>> #trait_ident<BaseMarker> for T {
            #(#trait_fns_direct_impl)*
        }
    };

    let gd_bridge_def = quote! {
        #[derive(GodotClass)]
        #[class(base = RefCounted, init)]
        pub struct #gd_bridge {}
    };

    let gd_bridge_fns = tweens_map.iter().flat_map(|(ty, TweensMap { methods, .. })| {
        let class_ident = class_ident.clone();

        methods.iter().flat_map(move |TweenFunc { func, alias, .. }| {
            let mut items = Vec::new();

            items.push(quote! {
                #[func]
                fn #func(node: Gd<#class_ident>, to: #ty, duration: f64) -> Gd<<#ty as TyToGdTween>::GdTween> {
                    let gd = <#ty as TyToGdTween>::GdTween::new_gd();
                    register_gd_handle! { node.#func(to, duration), gd }
                }
            });

            if let Some(alias) = alias {
                items.push(quote! {
                    #[func]
                    fn #alias(node: Gd<#class_ident>, to: #ty, duration: f64) -> Gd<<#ty as TyToGdTween>::GdTween> {
                        let gd = <#ty as TyToGdTween>::GdTween::new_gd();
                        register_gd_handle! { node.#func(to, duration), gd }
                    }
                });
            }
            items
        })
    });

    let gd_bridge_impl = quote! {
        #[godot_api]
        impl #gd_bridge {
            #(#gd_bridge_fns)*
        }
    };

    quote! {
        use super::*;

        #(#ty_datas_def)*

        #trait_def

        #trait_gd_impl

        #trait_direct_impl

        #gd_bridge_def

        #gd_bridge_impl
    }
}

fn data_ty_ident(class: &str, ty: &TweenType) -> Ident { format_ident!("{class}{}Data", ty) }

fn enum_ty_ident(class: &str, ty: &TweenType) -> Ident { format_ident!("{class}{}Kind", ty) }

fn property_to_gd_method(property_path: &str) -> Ident {
    format_ident!("do_{}", property_path.replace(":", "_"))
}

fn gd_tween_ty(ty: TweenType) -> &'static str {
    match ty {
        TweenType::f32 => "TweenPropertyF32",
        TweenType::f64 => "TweenPropertyF64",
        TweenType::i32 => "TweenPropertyI32",
        TweenType::i64 => "TweenPropertyI64",
        TweenType::Vector2 => "TweenPropertyVector2",
        TweenType::Vector2i => "TweenPropertyVector2i",
        TweenType::Vector3 => "TweenPropertyVector3",
        TweenType::Vector3i => "TweenPropertyVector3i",
        TweenType::Color => "TweenPropertyColor",
        TweenType::String => "TweenPropertyString",
    }
}

fn gd_ty(ty: TweenType) -> &'static str {
    match ty {
        TweenType::f32 | TweenType::f64 => "float",
        TweenType::i32 | TweenType::i64 => "int",
        TweenType::Vector2 => "Vector2",
        TweenType::Vector2i => "Vector2i",
        TweenType::Vector3 => "Vector3",
        TweenType::Vector3i => "Vector3i",
        TweenType::Color => "Color",
        TweenType::String => "String",
    }
}

fn gd_bridge_ty(class: &str) -> Ident { format_ident!("{class}Tweener") }
