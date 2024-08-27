//@ aux-build:format-string-proc-macro.rs

extern crate format_string_proc_macro;

fn main() {
    format_string_proc_macro::bad_format_args_captures!();
// { dg-error "" "" { target *-*-* } .-1 }
}

