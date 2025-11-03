#![feature(stmt_expr_attributes)]
#![feature(iterator_try_collect)]

mod classes_gdscript_bridge;
mod classes_property_data;
mod json_to_data;
mod tween_ty;
mod types_property_data;

use std::str::FromStr;

use convert_case::{Case, Casing};
use json_to_data::{ClassData, ClassJson, TweensMap, *};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use syn::{ext::IdentExt, *};
use tween_ty::*;

fn main() -> anyhow::Result<()> {
    let json_str = include_str!("./../../tweenable_properties.json");
    let classes_json: Vec<ClassJson> = serde_json::from_str(json_str)?;
    let mut classes = classes_json.into_iter().map(gen_class_data).collect::<Vec<_>>();
    classes.sort_by_key(|class| class.name.clone());

    classes_property_data::generate(&classes)?;
    types_property_data::generate(&classes)?;
    classes_gdscript_bridge::generate(&classes)?;

    Ok(())
}
