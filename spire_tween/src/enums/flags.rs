/*
use super::*;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Flags: i32 {
        const ConnectDeferred = 1;
        const ConnectOneShot = 4;
    }
}

impl TryFrom<i32> for Flags {
    type Error = ();

    fn try_from(ord: i32) -> Result<Self, ()> { Flags::from_bits(ord).ok_or(()) }
}

impl GodotConvert for Flags {
    type Via = Flags;
}

impl ToGodot for Flags {
    type ToVia<'v> = Flags;
    fn to_godot(&self) -> Self::ToVia<'_> { *self }
}

impl FromGodot for Flags {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> { Ok(via) }
}

#[allow(non_upper_case_globals)]
static Flags_CLASS_NAME: LazyLock<ClassName> =
    LazyLock::new(|| ClassName::alloc_next_ascii_rust_str(concat!("Spire.Flags")));

impl GodotType for Flags {
    type Ffi = i64;
    type ToFfi<'f> = i64;

    #[inline]
    fn to_ffi(&self) -> Self::ToFfi<'_> { self.bits() as i64 }

    #[inline]
    fn into_ffi(self) -> Self::Ffi { self.bits() as i64 }

    #[inline]
    fn try_from_ffi(ffi: Self::Ffi) -> Result<Self, ConvertError> {
        Self::try_from(ffi as i32).map_err(|_| FromGodotError::InvalidEnum.into_error(ffi))
    }

    #[inline]
    fn from_ffi(ffi: Self::Ffi) -> Self { Self::try_from(ffi as i32).unwrap_or_default() }

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
    fn class_name() -> ClassName { *Flags_CLASS_NAME }

    #[inline]
    fn property_hint_info() -> PropertyHintInfo { Self::var_hint() }

    #[inline]
    fn godot_type_name() -> String { Self::class_name().to_string() }

    #[inline]
    fn qualifies_as_special_none(_from_variant: &Variant) -> bool { false }
}

#[allow(non_upper_case_globals)]
static Flags_HINT_STRING: LazyLock<SafeGString> =
    LazyLock::new(|| SafeGString(GString::from("ConnectDeferred:1,ConnectOneShot:2")));

impl Var for Flags {
    fn get_property(&self) -> Self::Via { *self }

    fn set_property(&mut self, value: Self::Via) { *self = value; }

    fn var_hint() -> PropertyHintInfo {
        PropertyHintInfo {
            hint: PropertyHint::FLAGS,
            hint_string: Flags_HINT_STRING.0.clone(),
        }
    }
}

impl Export for Flags {}

plugin_execute_pre_main! {{
    bridge_registration_constants().lock().unwrap().push(|| {
        ExportConstant::new(
            Spire::class_name(),
            ConstantKind::Bitfield {
                name: "Flags".into(),
                flags: vec![
                    IntegerConstant::new("FLAGS_CONNECT_DEFERRED", Flags::ConnectDeferred.bits() as i64),
                    IntegerConstant::new("FLAGS_CONNECT_ONE_SHOT", Flags::ConnectOneShot.bits() as i64),
                ],
            },
        )
        .register();
    });
}}

plugin_add!(godot::private::__GODOT_DOCS_REGISTRY; {
    let mut docs = String::new();

    docs.push_str(docs::generate_enum_constant_docs(
        &["See [constant Object.CONNECT_DEFERRED]"], "FLAGS_CONNECT_DEFERRED", Flags::ConnectDeferred.bits()).as_str()
    );
    docs.push_str(docs::generate_enum_constant_docs(
        &["See [constant Object.CONNECT_ONE_SHOT]"], "FLAGS_CONNECT_ONE_SHOT", Flags::ConnectDeferred.bits()).as_str()
    );

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
*/
