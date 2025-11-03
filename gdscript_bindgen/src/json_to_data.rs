use super::*;

#[derive(Serialize, Deserialize)]
pub struct ClassJson {
    class:  String,
    tweens: Vec<TweenJson>,
    group:  Option<ClassGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct TweenJson {
    name: String,
    ty: TweenType,
    func: Option<String>,
    alias: Option<String>,
    get: Option<String>,
    set: Option<String>,
}

#[spire_enum::prelude::discriminant_generic_table]
#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub enum ClassGroup {
    #[default]
    Misc,
    #[serde(rename = "UI")]
    Control,
    #[serde(rename = "2D")]
    Node2D,
    #[serde(rename = "3D")]
    Node3D,
}

pub struct ClassData {
    pub name: String,
    pub ident: Ident,
    pub trait_ident: Ident,
    pub gdscript_bridge: Ident,
    #[allow(unused)]
    pub group: ClassGroup,
    pub tweens_map: GdScriptTypeDiscriminantTable<TweensMap>,
}

pub struct TweensMap {
    pub data_ty: Ident,
    pub enum_ty: Ident,
    pub methods: Vec<TweenFunc>,
}

#[derive(Clone)]
pub struct TweenFunc {
    pub property_path: String,
    pub alias: Option<(Ident, Ident)>,
    pub enum_discrim: Ident,
    pub get_expr: TokenStream,
    pub set_expr: TokenStream,
    pub bridge_fn_name: Ident,
    pub trait_fn_name: Ident,
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

        let bridge_fn_name = Ident::new_raw(&property_path, Span::call_site());

        let trait_fn_name = func
            .as_ref()
            .map(|s| Ident::new(&format!("do_{s}"), Span::call_site()))
            .unwrap_or_else(|| format_ident!("do_{}", bridge_fn_name));

        let enum_discrim = format_ident!("{}", name.to_case(Case::Pascal));

        let get_expr = {
            let expr = get
                .as_ref()
                .map(|str| TokenStream::from_str(str).unwrap())
                .unwrap_or_else(|| {
                    let get_ident = format_ident!("get_{}", name);
                    quote! { obj.#get_ident() }
                });

            match ty {
                TweenType::i32 => quote! { (#expr) as i64 },
                TweenType::f32 => quote! { (#expr) as f64 },
                _ => expr,
            }
        };

        let set_expr = set
            .as_ref()
            .map(|str| TokenStream::from_str(str).unwrap())
            .unwrap_or_else(|| {
                let set_ident = format_ident!("set_{}", name);

                match ty {
                    TweenType::f32 => quote! { obj.#set_ident(val as f32); },
                    TweenType::i32 => quote! { obj.#set_ident(val as i32); },
                    TweenType::String => quote! { obj.#set_ident(&val); },
                    _ => quote! { obj.#set_ident(val); },
                }
            });

        let alias = alias
            .as_ref()
            .map(|s| (Ident::new_raw(s, Span::call_site()), format_ident!("do_{s}")));

        Self {
            property_path,
            enum_discrim,
            get_expr,
            set_expr,
            alias,
            bridge_fn_name,
            trait_fn_name,
        }
    }

    fn indexed(
        inner_field: &str,
        TweenJson {
            name,
            ty,
            func,
            alias,
            get,
            set,
        }: &TweenJson,
    ) -> Self {
        let property_path = format!("{name}:{inner_field}");

        let bridge_fn_name = Ident::new(&property_path.replace(":", "_"), Span::call_site());
        let trait_fn_name = func
            .as_ref()
            .map(|s| format_ident!("do_{s}_{inner_field}"))
            .unwrap_or_else(|| format_ident!("do_{}", bridge_fn_name));

        let inner_field_ident = format_ident!("{inner_field}");
        let enum_discrim = format_ident!("{}{}", name.to_case(Case::Pascal), inner_field.to_uppercase());

        let get_ident = format_ident!("get_{}", name);

        let get_expr = {
            let expr = if let Some(get) = get {
                let get_ts = TokenStream::from_str(get).unwrap();
                quote! { #get_ts.#inner_field_ident }
            } else {
                quote! { obj.#get_ident().#inner_field_ident }
            };

            match ty {
                TweenType::Vector2i | TweenType::Vector3i => quote! { (#expr) as i64 },
                TweenType::Vector2 | TweenType::Vector3 | TweenType::Color => {
                    quote! { (#expr) as f64 }
                }
                _ => expr,
            }
        };

        let set_expr = if let Some(set) = set {
            let set_ts = TokenStream::from_str(set).unwrap();

            match ty {
                TweenType::Vector2i | TweenType::Vector3i => {
                    quote! {
                        let mut prop_val = #get_expr;
                        prop_val.#inner_field_ident = val as i32;
                        let val = prop_val;
                        #set_ts
                    }
                }
                TweenType::Vector2 | TweenType::Vector3 | TweenType::Color => {
                    quote! {
                        let mut prop_val = #get_expr;
                        prop_val.#inner_field_ident = val as f32;
                        let val = prop_val;
                        #set_ts
                    }
                }
                _ => {
                    quote! {
                        let mut prop_val = #get_expr;
                        prop_val.#inner_field_ident = val;
                        let val = prop_val;
                        #set_ts
                    }
                }
            }
        } else {
            let set_ident = format_ident!("set_{}", name);

            match ty {
                TweenType::Vector2i | TweenType::Vector3i => {
                    quote! {
                        let mut prop_val = obj.#get_ident();
                        prop_val.#inner_field_ident = val as i32;
                        obj.#set_ident(prop_val);
                    }
                }
                TweenType::Vector2 | TweenType::Vector3 | TweenType::Color => {
                    quote! {
                        let mut prop_val = obj.#get_ident();
                        prop_val.#inner_field_ident = val as f32;
                        obj.#set_ident(prop_val);
                    }
                }
                _ => {
                    quote! {
                        let mut prop_val = obj.#get_ident();
                        prop_val.#inner_field_ident = val;
                        obj.#set_ident(prop_val);
                    }
                }
            }
        };

        let alias = alias
            .as_ref()
            .map(|s| (format_ident!("{s}_{inner_field}"), format_ident!("do_{s}_{inner_field}")));

        Self {
            property_path,
            enum_discrim,
            get_expr,
            set_expr,
            alias,
            bridge_fn_name,
            trait_fn_name,
        }
    }
}

pub fn gen_class_data(json: ClassJson) -> ClassData {
    let ident = format_ident!("{}", json.class);
    let trait_ident = format_ident!("SpireDo{ident}");
    let gdscript_bridge = format_ident!("Do{}", json.class);
    let tweens_map = gen_class_type_tweens_map(&json);

    ClassData {
        name: json.class.clone(),
        ident,
        trait_ident,
        gdscript_bridge,
        tweens_map,
        group: json.group.unwrap_or_default(),
    }
}

fn gen_class_type_tweens_map(ClassJson { class, tweens, .. }: &ClassJson) -> GdScriptTypeDiscriminantTable<TweensMap> {
    let mut type_tweens_map = GdScriptTypeDiscriminantTable::from_fn(|ty| TweensMap {
        data_ty: class_property_data_ident(class, &ty),
        enum_ty: class_property_data_enum_ident(class, &ty),
        methods: Vec::new(),
    });

    for tween_json in tweens {
        let gd_ty = GdScriptType::from(tween_json.ty);
        type_tweens_map[gd_ty].methods.push(TweenFunc::direct(tween_json));

        match tween_json.ty {
            TweenType::Vector2 => {
                type_tweens_map[GdScriptType::float]
                    .methods
                    .extend([TweenFunc::indexed("x", tween_json), TweenFunc::indexed("y", tween_json)]);
            }
            TweenType::Vector2i => {
                type_tweens_map[GdScriptType::int]
                    .methods
                    .extend([TweenFunc::indexed("x", tween_json), TweenFunc::indexed("y", tween_json)]);
            }
            TweenType::Vector3 => {
                type_tweens_map[GdScriptType::float].methods.extend([
                    TweenFunc::indexed("x", tween_json),
                    TweenFunc::indexed("y", tween_json),
                    TweenFunc::indexed("z", tween_json),
                ]);
            }
            TweenType::Vector3i => {
                type_tweens_map[GdScriptType::int].methods.extend([
                    TweenFunc::indexed("x", tween_json),
                    TweenFunc::indexed("y", tween_json),
                    TweenFunc::indexed("z", tween_json),
                ]);
            }
            TweenType::Color => {
                type_tweens_map[GdScriptType::float].methods.extend([
                    TweenFunc::indexed("r", tween_json),
                    TweenFunc::indexed("g", tween_json),
                    TweenFunc::indexed("b", tween_json),
                    TweenFunc::indexed("a", tween_json),
                ]);
            }
            | TweenType::f32 | TweenType::f64 | TweenType::i32 | TweenType::i64 | TweenType::String => {}
        }
    }

    type_tweens_map
}
