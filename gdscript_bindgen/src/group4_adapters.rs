use super::*;

pub fn methods(class: &Ident) -> TokenStream {
    match class.to_string().as_str() {
        "Node2D" | "Node3D" | "Control" => paths(class),
        "MarginContainer" => quote! {
            /// Tweens effective theme margins to (left, right, top, bottom).
            /// Captures on first step and writes rounded local theme overrides.
            /// Signed values are supported. Already registered.
            #[func]
            fn margins(node: Gd<MarginContainer>, to: Vector4i, duration: f64) -> Gd<SpireMethodFloat> {
                gd_from_native_tween(node.do_margins(to, duration).register())
            }
        },
        _ => TokenStream::new(),
    }
}

fn paths(class: &Ident) -> TokenStream {
    let array = if class == "Node3D" { quote!(PackedVector3Array) } else { quote!(PackedVector2Array) };
    let mut out = TokenStream::new();
    for (method, native, configured) in [
        ("path", "do_path", false),
        ("global_path", "do_global_path", false),
        ("path_with_options", "do_path", true),
        ("global_path_with_options", "do_global_path", true),
    ] {
        let method = format_ident!("{method}");
        let native = format_ident!("{native}");
        let argument = configured.then(|| quote!(, options: Gd<PathOptions>));
        let options = if configured { quote!(options.bind().clone()) } else { quote!(PathOptions::default()) };
        out.extend(quote! {
            /// Visits waypoints by distance. path uses local coordinates;
            /// global_path uses world coordinates. Captures start on first step.
            /// The with_options variants copy a SpirePathOptions configuration.
            /// Other variants select linear defaults. Cubic inputs use
            /// triples: destination, preceding handle, destination handle.
            /// Supports closure, axis locks, and optional look-at orientation.
            /// This sets transforms directly; it does not perform physics motion.
            /// Returns an already registered float progress tween.
            #[func]
            fn #method(node: Gd<#class>, waypoints: #array, duration: f64 #argument) -> Gd<SpireMethodFloat> {
                let options = #options;
                gd_from_native_tween(node.#native(waypoints.to_vec(), duration, options).register())
            }
        });
    }
    out
}
