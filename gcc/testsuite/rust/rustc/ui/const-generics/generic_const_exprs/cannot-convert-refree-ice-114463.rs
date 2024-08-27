// issue: rust-lang/rust#114463
// ICE cannot convert `ReFree ..` to a region vid
#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn bug<'a>() {
    [(); (|_: &'a u8| (), 0).1];
// { dg-error "" "" { target *-*-* } .-1 }
}

pub fn main() {}

