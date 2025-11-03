/*
#![allow(unused_imports)]
use std::{
    hash::{BuildHasherDefault, DefaultHasher, RandomState},
    time::{Duration, Instant},
};

use super::*;
*/

/*
pub(crate) static RESULTS: scc::HashMap<
    &'static str,
    Vec<Duration>,
    BuildHasherDefault<DefaultHasher>,
> = scc::HashMap::with_hasher(BuildHasherDefault::new());
*/

/*
#[allow(unused_macros)]
macro_rules! benchmark {
    ($Caller: ident . $FnName: path [$($TT:tt)*]) => {{
        $Caller.$FnName($($TT)*)
    }};

    ($FnName: path [$($TT:tt)*]) => {{
        $FnName($($TT)*)
    }};

    ($Mod: path => $Caller: ident . $FnName: ident ($($TT:tt)*) $($OtherTT: tt)*) => {{
        $Caller.$FnName($($TT)*) $($OtherTT)*
    }};
}
*/

/*
macro_rules! benchmark {
    ($Caller: ident . $FnName: path [$($TT:tt)*]) => {{
        let start = std::time::Instant::now();

        let res = $Caller.$FnName($($TT)*);

        let duration = start.elapsed();

        let fn_name = std::any::type_name_of_val(&$FnName);
        crate::benchmarking::RESULTS.entry_sync(fn_name).or_default().push(duration);

        res
    }};

    ($FnName: path [$($TT:tt)*]) => {{
        let start = std::time::Instant::now();

        let res = $FnName($($TT)*);

        let duration = start.elapsed();

        let fn_name = std::any::type_name_of_val(&$FnName);
        crate::benchmarking::RESULTS.entry_sync(fn_name).or_default().push(duration);

        res
    }};

    ($Mod: path => $Caller: ident . $FnName: ident ($($TT:tt)*) $($OtherTT: tt)*) => {{
        let start = std::time::Instant::now();

        let res = $Caller.$FnName($($TT)*) $($OtherTT)* ;

        let duration = start.elapsed();

        let fn_name = std::any::type_name_of_val(&<$Mod>::$FnName);
        crate::benchmarking::RESULTS.entry_sync(fn_name).or_default().push(duration);

        res
    }};
}
*/
/*

pub(crate) use benchmark;

pub fn print_results() {
    let mut builder = String::new();

    RESULTS.iter_sync(|name, times| {
        let total: Duration = times.iter().sum();
        let count = times.len() as u32;
        let avg = total / count;

        builder.push_str(&format!("{name}: ran {count} times, avg: {avg:?}, total: {total:?}\n"));

        true
    });

    godot_print!("{builder}");
}

*/
