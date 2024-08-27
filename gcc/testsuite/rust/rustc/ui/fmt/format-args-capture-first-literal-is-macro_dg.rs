//@ aux-build:format-string-proc-macro.rs

#[macro_use]
extern crate format_string_proc_macro;

macro_rules! identity_mbe {
    ($tt:tt) => {
        $tt
    };
}

fn main() {
    let a = 0;

    format!(identity_pm!("{a}"));
// { dg-error "" "" { target *-*-* } .-1 }
    format!(identity_mbe!("{a}"));
// { dg-error "" "" { target *-*-* } .-1 }
    format!(concat!("{a}"));
// { dg-error "" "" { target *-*-* } .-1 }
}

