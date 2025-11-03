macro_rules! register_enum {
    (
        [GD = $GdIdent: literal]
        $RustIdent: ident {
            $(
                $(#[doc = $VarDoc: literal])*
                [RS = $VarRs: literal, GD = $VarGd: literal]
                $( #[$($VarAttrs: tt)*] )*
                $VarIdent: ident = $VarValue: expr
            ),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        #[repr(i32)]
        pub enum $RustIdent {
            $(
                $(#[doc = $VarDoc])*
                $( #[$($VarAttrs)*] )*
                $VarIdent = $VarValue,
            )*
        }

        impl TryFrom<i32> for $RustIdent {
            type Error = !;

            fn try_from(ord: i32) -> Result<Self, !> {
                match ord {
                    $( $VarValue => Ok(Self::$VarIdent), )*
                    invalid => {
                        godot_error!(
                            "Invalid value `{invalid}` for enum of type `{}`, using default value instead.",
                                stringify!($RustIdent)
                        );
                        Ok(Default::default())
                    },
                }
            }
        }

        impl GodotConvert for $RustIdent {
            type Via = $RustIdent;
        }

        impl ToGodot for $RustIdent {
            type ToVia<'v> = $RustIdent;
            fn to_godot(&self) -> Self::ToVia<'_> { *self }
        }

        impl FromGodot for $RustIdent {
            fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> { Ok(via) }
        }

        #[allow(non_upper_case_globals)]
        static ${concat($RustIdent, _CLASS_NAME)}: LazyLock<ClassName> =
            LazyLock::new(|| ClassName::alloc_next_ascii_rust_str(concat!("Spire.",$GdIdent)));

        impl GodotType for $RustIdent {
            type Ffi = i64;
            type ToFfi<'f> = i64;

            #[inline]
            fn to_ffi(&self) -> Self::ToFfi<'_> { *self as i64 }

            #[inline]
            fn into_ffi(self) -> Self::Ffi { self as i64 }

            #[inline]
            fn try_from_ffi(ffi: Self::Ffi) -> Result<Self, ConvertError> {
                match Self::try_from(ffi as i32) {
                    Ok(val) => Ok(val),
                }
            }

            #[inline]
            fn from_ffi(ffi: Self::Ffi) -> Self {
                match Self::try_from(ffi as i32) {
                    Ok(val) => val,
                }
            }

            #[inline]
            fn param_metadata() -> GDExtensionClassMethodArgumentMetadata {
                godot::sys::GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT32
            }

            #[inline]
            fn property_info(property_name: &str) -> PropertyInfo {
                PropertyInfo {
                    variant_type: VariantType::INT,
                    class_name: Self::class_name(),
                    property_name: property_name.into(),
                    hint_info: Self::var_hint(),
                    usage: PropertyUsageFlags::CLASS_IS_ENUM,
                }
            }

            #[inline]
            fn class_name() -> ClassName { ${concat($RustIdent, _CLASS_NAME)}.clone() }

            #[inline]
            fn property_hint_info() -> PropertyHintInfo { Self::var_hint() }

            #[inline]
            fn godot_type_name() -> String { Self::class_name().to_string() }

            #[inline]
            fn qualifies_as_special_none(_from_variant: &Variant) -> bool { false }
        }

        #[allow(non_upper_case_globals)]
        static ${concat($RustIdent, _HINT_STRING)}: LazyLock<SafeGString> = LazyLock::new(||
            SafeGString(GString::from(
                concat!( $( $VarRs, ":", $VarValue ),* )
            ))
        );

        impl Var for $RustIdent {
            fn get_property(&self) -> Self::Via { *self }

            fn set_property(&mut self, value: Self::Via) { *self = value; }

            fn var_hint() -> PropertyHintInfo {
                PropertyHintInfo {
                    hint: PropertyHint::ENUM,
                    hint_string: ${concat($RustIdent, _HINT_STRING)}.0.clone(),
                }
            }
        }

        impl Export for $RustIdent {}

        plugin_execute_pre_main! {{
            bridge_registration_constants().lock().unwrap().push(|| {
                ExportConstant::new(
                    Spire::class_name(),
                    ConstantKind::Enum {
                        name: $GdIdent.into(),
                        enumerators: vec![
                            $( IntegerConstant::new($VarGd, $VarValue) ),*
                        ],
                    },
                )
                .register();
            });
        }}

        plugin_add!(godot::private::__GODOT_DOCS_REGISTRY; {
            let mut docs = String::new();

            $(
                docs.push_str($crate::enums::docs::generate_enum_constant_docs(
                    &[ $($VarDoc,)* ], $VarGd, <$RustIdent>::$VarIdent as i32).as_str()
                );
            )*

            godot::private::DocsPlugin::new::<Spire>(
                godot::private::DocsItem::InherentImpl(
                    godot::private::InherentImplDocs {
                        methods : "" ,
                        signals : "" ,
                        constants : Box::leak(docs.into_boxed_str())
                    }
                )
            )
        });
    };
}

pub(crate) use register_enum;
