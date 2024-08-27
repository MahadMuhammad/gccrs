//@ aux-build:format-string-proc-macro.rs

#[macro_use]
extern crate format_string_proc_macro;

macro_rules! def_site {
    () => { "{foo}" } // { dg-error "" "" { target *-*-* } }
}

macro_rules! call_site {
    ($fmt:literal) => { $fmt }
}

fn main() {
    format!(concat!("{foo}"));         // { dg-error "" "" { target *-*-* } }
    format!(concat!("{ba", "r} {}"), 1);     // { dg-error "" "" { target *-*-* } }

    format!(def_site!());
    format!(call_site!("{foo}")); // { dg-error "" "" { target *-*-* } }

    format!(foo_with_input_span!("")); // { dg-error "" "" { target *-*-* } }
}

