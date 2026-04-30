/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};

use super::{FuncDefinition, markdown_converter};

#[derive(Default)]
struct XmlParagraphs {
    /// XML content, as BBCode, to be used in `description` tag: `<description>VALUE</description>`.
    description_content: String,
}

pub struct InherentImplXmlDocs {
    pub method_xml_elems: String,
}

/// Returns code containing the doc information of a `#[godot_api] impl MyClass` declaration.
pub fn document_inherent_impl(functions: &[FuncDefinition]) -> InherentImplXmlDocs {
    let method_xml_elems = functions.iter().filter_map(format_method_xml).collect::<String>();

    InherentImplXmlDocs { method_xml_elems }
}

/// `///` is expanded to `#[doc = "…"]`.
///
/// This function goes through and extracts the "…" part.
fn extract_docs_from_attributes(doc: &[venial::Attribute]) -> impl Iterator<Item = String> + '_ {
    doc.iter()
        // Find #[doc].
        .filter(|x| x.get_single_path_segment().is_some_and(|x| x == "doc"))
        // Limit to occurrences with syntax #[doc = "…"].
        .filter_map(|x| match &x.value {
            venial::AttributeValue::Equals(_, doc) => Some(doc),
            _ => None,
        })
        .flat_map(|doc| {
            doc.iter().map(|token_tree| {
                let str = token_tree.to_string();
                litrs::StringLit::parse(str.clone())
                    .map_or(str, |parsed| parsed.value().to_string())
            })
        })
}

fn xml_escape(value: String) -> String {
    // Most strings have no special characters, so this check helps avoid unnecessary string copying.
    if !value.contains(['&', '<', '>', '"', '\'']) {
        return value;
    }

    let mut result = String::with_capacity(value.len());

    for c in value.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            c => result.push(c),
        }
    }

    result
}

/// Extracts docs from attributes and groups them in three Strings:
/// user documentation content and paragraphs annotated with `@deprecated` or `@experimental` tags.
fn docs_with_attributes(doc: &[venial::Attribute]) -> (String, String, String) {
    let (mut docs, mut deprecated, mut experimental) = (String::new(), String::new(), String::new());

    // Allows to compare the current bucket (the one we put current paragraph in) with docs one.
    let docs_bucket = std::ptr::from_ref(&docs);
    let mut current_bucket: &mut String = &mut docs;

    for line in extract_docs_from_attributes(doc) {
        let trimmed = line.trim_start();

        // End of the paragraph (`#[doc=""]` or `///`) .
        if trimmed.is_empty() {
            // Switch back from attribute docs to user docs when paragraph ends.
            // Don't double newlines after XML attribute tags descriptions.
            if !std::ptr::eq(current_bucket, docs_bucket) {
                current_bucket = &mut docs;
            } else {
                current_bucket.push('\n');
            }
            continue;
        }

        // Check for `/// @deprecated` ... or `/// @experimental`
        if trimmed.starts_with("@deprecated") {
            current_bucket = &mut deprecated;
            current_bucket.push_str(trimmed.trim_start_matches("@deprecated"));
        } else if trimmed.starts_with("@experimental") {
            current_bucket = &mut experimental;
            current_bucket.push_str(trimmed.trim_start_matches("@experimental"));
        } else {
            current_bucket.push_str(&line);
            current_bucket.push('\n');
        }
    }

    (docs, deprecated, experimental)
}

/// Converts attribute docs to form suitable for Godot's consumption.
///
/// See also: [`XmlParagraphs`].
fn attribute_docs_to_xml_paragraphs(doc: &[venial::Attribute]) -> Option<XmlParagraphs> {
    let (docs, deprecated, experimental) = docs_with_attributes(doc);

    if docs.is_empty() && deprecated.is_empty() && experimental.is_empty() {
        return None;
    }

    let to_bbcode: fn(String) -> Option<String> =
        |piece| (!piece.is_empty()).then(|| markdown_converter::to_bbcode(&piece));

    Some(XmlParagraphs {
        description_content: to_bbcode(docs).map(xml_escape).unwrap_or_default(),
    })
}

fn format_venial_params_xml(params: &venial::Punctuated<venial::FnParam>) -> String {
    let non_receiver_params = params.iter().filter_map(|(param, _punct)| match param {
        venial::FnParam::Receiver(_) => None,
        venial::FnParam::Typed(p) => Some((&p.name, &p.ty)),
    });

    format_params_xml(non_receiver_params)
}

fn format_params_xml<'a, 'b>(params: impl Iterator<Item = (&'a Ident, &'b venial::TypeExpr)>) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    for (index, (name, ty)) in params.enumerate() {
        write!(
            output,
            r#"<param index="{index}" name="{name}" type="{ty}" />"#,
            name = xml_escape(name.to_string()),
            ty = xml_escape(ty.to_token_stream().to_string()),
        )
        .expect("write to string failed");
    }
    output
}

pub fn format_method_xml(method: &FuncDefinition) -> Option<String> {
    let XmlParagraphs { description_content } = attribute_docs_to_xml_paragraphs(&method.external_attributes)?;

    let name = method.rust_ident().to_string();
    let name = xml_escape(name);

    let signature = &method.signature_info;

    let return_ty = signature.return_type.to_token_stream().to_string();
    let return_ty = xml_escape(return_ty);

    let param_names_and_types = signature.param_idents.iter().zip(&signature.param_types);
    let params = format_params_xml(param_names_and_types);

    Some(format!(
        r#"
<method name="{name}">
  <return type="{return_ty}" />
  {params}
  <description>
  {description_content}
  </description>
</method>
"#
    ))
}
