// issue rust-lang/rust#121463
// ICE non-ADT in struct pattern
#![feature(box_patterns)]

fn main() {
    let mut a = E::StructVar { boxed: Box::new(5_i32) };
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    match a {
        E::StructVar { box boxed } => { }
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    }
}

