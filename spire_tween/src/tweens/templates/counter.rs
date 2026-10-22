use super::*;

fn format_counter(value: i64, separator: &str) -> GString {
    let digits = value.unsigned_abs().to_string();
    let mut result = String::new();
    if value < 0 {
        result.push('-');
    }
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            result.push_str(separator);
        }
        result.push(digit);
    }
    GString::from(&result)
}

macro_rules! counter {
    ($trait:ident, $class:ty) => {
        /// Integer-to-text template. The starting integer is explicit; existing
        /// text is not parsed. Separators group every three digits and can contain
        /// Unicode; an empty separator disables grouping. Replaces the whole text
        /// (including BBCode on RichTextLabel), using ordinary integer interpolation.
        /// Returns an unregistered method tween with receiver lifetime ownership.
        pub trait $trait {
            fn do_counter(
                &self,
                from: i64,
                to: i64,
                duration: f64,
                thousands_separator: GString,
            ) -> SpireTween<LerpMethodData<i64>>;
        }
        impl<C: Inherits<$class> + Inherits<Object>> $trait for Gd<C> {
            fn do_counter(
                &self,
                from: i64,
                to: i64,
                duration: f64,
                thousands_separator: GString,
            ) -> SpireTween<LerpMethodData<i64>> {
                let mut node = self.clone().upcast::<$class>();
                let owner = ObjectOrNode::from_unchecked_object(node.clone().upcast());
                let separator = thousands_separator.to_string();
                let callable = Callable::from_fn("spire_counter", move |args| {
                    node.set_text(&format_counter(args[0].to::<i64>(), &separator));
                    Variant::nil()
                });
                let mut tween = SpireTween::<LerpMethodData<i64>>::new(callable, from, to, duration);
                tween.t.owner = Some(owner);
                tween
            }
        }
    };
}
counter!(DoLabelCounter, Label);
counter!(DoRichTextLabelCounter, RichTextLabel);
counter!(DoLabel3DCounter, Label3D);
