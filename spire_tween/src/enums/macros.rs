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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, GodotConvert, Var, Export)]
        #[godot(via = i32)]
        #[repr(i32)]
        pub enum $RustIdent {
            $(
                $(#[doc = $VarDoc])*
                $( #[$($VarAttrs)*] )*
                $VarIdent = $VarValue,
            )*
        }

        // SAFETY: Copy + #[repr(i32)] enum with no interior mutability. Each instance
        // is truly independent — required for `#[opt(default = ...)]` to accept it.
        unsafe impl ::godot::meta::GodotImmutable for $RustIdent {}

        #[cfg(feature = "standalone")]
        shard_execute_pre_main! {{
            bridge_registration_constants().lock().unwrap().1.push(|| {
                ExportConstant::new(
                    <Spire as ::godot::obj::GodotClass>::class_id(),
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

        #[cfg(feature = "standalone")]
        shard_add!(godot::private::__GODOT_DOCS_REGISTRY; {
            let mut docs = String::new();

            $(
                docs.push_str($crate::enums::docs::generate_enum_constant_docs(
                    &[ $($VarDoc,)* ], $VarGd, <$RustIdent>::$VarIdent as i32).as_str()
                );
            )*

            godot::private::DocsShard::new::<Spire>(
                godot::private::DocsItem::InherentImpl(
                    godot::private::InherentImplDocs {
                        methods_xml : "" ,
                        signals_xml : "" ,
                        constants_xml : Box::leak(docs.into_boxed_str())
                    }
                )
            )
        });
    };
}

pub(crate) use register_enum;
