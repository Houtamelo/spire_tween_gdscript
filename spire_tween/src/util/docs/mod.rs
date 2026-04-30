mod extract_docs;
mod func;
mod markdown_converter;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use venial::Item;

pub use self::{extract_docs::format_method_xml, func::FuncDefinition};

fn ident(s: &str) -> Ident { format_ident!("{s}") }

fn safe_ident(s: &str) -> Ident {
    // See also: https://doc.rust-lang.org/reference/keywords.html
    match s {
        // Lexer
        | "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
        | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while"

        // Lexer 2018+
        | "async" | "await" | "dyn"

        // Reserved
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield"

        // Reserved 2018+
        | "try"
        => format_ident!("{}_", s),

        _ => ident(s)
    }
}
