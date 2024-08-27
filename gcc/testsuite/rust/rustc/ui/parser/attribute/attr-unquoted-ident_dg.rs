//@ compile-flags: -Zdeduplicate-diagnostics=yes

#![allow(unexpected_cfgs)]

fn main() {
    #[cfg(key=foo)]
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    println!();
    #[cfg(key="bar")]
    println!();
    #[cfg(key=foo bar baz)]
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    println!();
}

// Don't suggest surrounding `$name` or `nickname` with quotes:

macro_rules! make {
    ($name:ident) => { #[doc(alias = $name)] pub struct S; }
// { dg-error "" "" { target *-*-* } .-1 }
}

make!(nickname); // { dg-note "" "" { target *-*-* } }

