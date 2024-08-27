//@ aux-build:format-string-proc-macro.rs

extern crate format_string_proc_macro;

fn main() {
    format_string_proc_macro::respan_to_invalid_format_literal!("¡");
// { dg-error "" "" { target *-*-* } .-1 }
    format_args!(r#concat!("¡        {"));
// { dg-error "" "" { target *-*-* } .-1 }
}

